use std::ffi::OsString;
use std::path::Path;
use std::sync::Arc;
use std::thread;

use config::{Config, File, FileFormat};
use dirs::config_dir;
use druid::im::{vector, Vector};
use druid::widget::{prelude::*, Split};
use druid::widget::{Container, Either, Label, Maybe, Scroll, WidgetExt};
use druid::{
    AppDelegate, AppLauncher, ArcStr, Color, Command, Data, DelegateCtx, FontDescriptor,
    FontFamily, FontWeight, Handled, Key, Lens, Selector, Target, WindowDesc,
};
use notmuch::{Database, DatabaseMode};

mod mail;
mod ui;

use crate::mail::Thread;

const SEARCH_CHANGE: Selector<ArcStr> = Selector::new("search-change");
const LOAD_THREAD: Selector<Arc<Thread>> = Selector::new("load-thread");
const MARK_READ: Selector<Arc<Thread>> = Selector::new("mark-read");
const UI_FONT: Key<FontDescriptor> = Key::new("org.westwork.seneca.ui-font");
const UI_FONT_LARGE: Key<FontDescriptor> = Key::new("org.westwork.seneca.ui-font-large");
const UI_FONT_LIGHT: Key<FontDescriptor> = Key::new("org.westwork.seneca.ui-font-light");
const THREAD_BACKGROUND_COLOR: Key<Color> = Key::new("org.westwork.seneca.background-color");
const THREAD_SELECTED_COLOR: Key<Color> = Key::new("org.westwork.seneca.thread-selected-color");
const BORDER_COLOR: Key<Color> = Key::new("org.westwork.seneca.border-color");
const SEARCH_BACKGROUND_COLOR: Key<Color> = Key::new("org.westwork.seneca.search-background-color");
const SEARCH_SELECTED_COLOR: Key<Color> = Key::new("org.westwork.seneca.search-selected-color");

#[derive(Data, Lens, Clone)]
pub struct MailData {
    threads: Vector<Arc<Thread>>,
    searches: Searches,
    done_loading: bool,
    loaded_thread: Option<Arc<Thread>>,
}

#[derive(Data, Lens, Clone)]
pub struct Searches {
    search_list: Vector<(ArcStr, ArcStr)>,
    selected: ArcStr,
}

struct Delegate {
    database: OsString,
}

impl AppDelegate<MailData> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut MailData,
        _env: &Env,
    ) -> Handled {
        if let Some(query) = cmd.get(SEARCH_CHANGE) {
            data.done_loading = false;
            let event_sink = ctx.get_external_handle();
            let query_clone = query.clone();
            let db_loc = self.database.clone();
            let _detached_thread =
                thread::spawn(move || mail::load_mail(query_clone, event_sink, &db_loc));
            return Handled::Yes;
        }

        if let Some(mut to_load) = cmd.get(LOAD_THREAD) {
            let loading_thread = mail::load_thread_from_disk(to_load.clone());
            to_load = &loading_thread.clone();
            data.loaded_thread = Some(loading_thread.clone());
            return Handled::Yes;
        }

        if let Some(to_mark) = cmd.get(MARK_READ) {
            let db = Database::open(Path::new(&self.database), DatabaseMode::ReadWrite).unwrap();
            let nm_messages = db
                .create_query(&format!("thread:{}", to_mark.id))
                .unwrap()
                .search_messages()
                .unwrap();
            for message in nm_messages {
                message.remove_tag("unread").unwrap();
            }
        }

        Handled::No
    }
}

fn main() {
    let mut config_file = config_dir().unwrap();
    config_file.push("seneca/config.toml");
    let config_builder = Config::builder().add_source(File::new(
        config_file
            .to_str()
            .expect("Config file path isn't unicode??"),
        FileFormat::Toml,
    ));

    let config = config_builder.build().expect("Error reading config file");
    let selected_search = Arc::from("tag:inbox");

    let search_mail = MailData {
        threads: Vector::new(),
        searches: Searches {
            search_list: vector![
                (Arc::from("Inbox"), Arc::clone(&selected_search)),
                (Arc::from("Github"), Arc::from("tag:github"))
            ],
            selected: selected_search,
        },
        done_loading: false,
        loaded_thread: None,
    };

    let main_window = WindowDesc::new(root_widget())
        .title("Seneca")
        .window_size((1000.0, 500.0));

    let db_osstr: OsString = config
        .get_string("db-location")
        .expect("No notmuch database in config file.")
        .to_string()
        .into();

    let launcher = AppLauncher::with_window(main_window);
    let event_sink = launcher.get_external_handle();
    let db_clone = db_osstr.clone();

    thread::spawn(move || mail::load_mail(Arc::from("tag:inbox"), event_sink, &db_clone));

    launcher
        .log_to_console()
        .delegate(Delegate { database: db_osstr })
        .configure_env(move |env: &mut Env, _app: &MailData| {
            env.set(
                THREAD_BACKGROUND_COLOR,
                get_color_from_config("thread-background-color", &config),
            );
            env.set(
                THREAD_SELECTED_COLOR,
                get_color_from_config("thread-selected-color", &config),
            );
            env.set(BORDER_COLOR, get_color_from_config("border-color", &config));
            env.set(
                SEARCH_BACKGROUND_COLOR,
                get_color_from_config("search-background-color", &config),
            );
            env.set(
                SEARCH_SELECTED_COLOR,
                get_color_from_config("search-selected-color", &config),
            );
            env.set(
                UI_FONT,
                FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(13.0),
            );
            env.set(
                UI_FONT_LARGE,
                FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(22.0),
            );
            env.set(
                UI_FONT_LIGHT,
                FontDescriptor::new(FontFamily::SYSTEM_UI)
                    .with_size(13.0)
                    .with_weight(FontWeight::LIGHT),
            );
            env.set(druid::theme::TEXT_COLOR, Color::BLACK);
            env.set(druid::theme::WIDGET_PADDING_HORIZONTAL, 1.);
        })
        .launch(search_mail)
        .expect("Failed to launch Seneca");
}

fn get_color_from_config(key: &str, config: &Config) -> Color {
    let color_table = config.get_table(key).expect(&format!(
        "No color theme found in config file. Missing key: {}.",
        key
    ));
    Color::rgb8(
        color_table.get("r").unwrap().clone().into_int().unwrap() as u8,
        color_table.get("g").unwrap().clone().into_int().unwrap() as u8,
        color_table.get("b").unwrap().clone().into_int().unwrap() as u8,
    )
}

fn root_widget() -> impl Widget<MailData> {
    let search_sidebar = ui::search_list::search_sidebar();
    let thread_widget = ui::thread_list::thread_list();
    let loading_widget = Label::new("Loading...").center();

    Split::columns(
        Container::new(Scroll::new(search_sidebar).vertical().background(SEARCH_BACKGROUND_COLOR)),
        Split::columns(
            Either::new(
                |data, _env| data.done_loading,
                thread_widget,
                loading_widget,
            ),
            Maybe::or_empty(|| mail::mail_layout()).lens(MailData::loaded_thread),
        )
        .split_point(0.3)
        .bar_size(0.0)
    )
    .split_point(0.15)
    .bar_size(0.0)
}
