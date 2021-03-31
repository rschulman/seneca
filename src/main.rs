use chrono::{NaiveDateTime, Utc};
use druid::im::{vector, Vector};
use druid::widget::prelude::*;
use druid::widget::{ClipBox, CrossAxisAlignment, Flex, Label, LensWrap, List, Scroll, WidgetExt};
use druid::{lens, AppLauncher, Color, Data, Lens, UnitPoint, WindowDesc};
use home::home_dir;
use itertools::Itertools;
use notmuch::{Database, DatabaseMode};

//mod mail;

#[derive(Data, Lens, Clone)]
struct MailData {
    emails: Vector<Mail>,
}

#[derive(Clone, Data)]
struct Mail {
    sender: String,
    recipients: Vector<String>,
    date: String,
    subject: String,
    body: String,
}

fn main() {
    let mut mail_path = home_dir().unwrap();
    mail_path.push("Personal");
    mail_path.push(".mail");
    let db = Database::open(
        &"/home/ross/Personal/.mail/".to_string(),
        DatabaseMode::ReadWrite,
    )
    .unwrap();
    let inbox = db.create_query("tag:inbox").unwrap();
    let threads = inbox.search_threads().unwrap();

    let mut search_mail = MailData {
        emails: Vector::new(),
    };

    for thread in threads {
        search_mail.emails.push_back(Mail {
            sender: thread.authors().iter().join(","),
            recipients: Vector::new(),
            date: NaiveDateTime::from_timestamp(thread.newest_date(), 0)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            subject: thread.subject().into(),
            body: "".to_string(),
        })
    }

    let main_window = WindowDesc::new(root_widget(&search_mail.emails))
        .title("Seneca")
        .window_size((400.0, 400.0));

    //let mail_state = check_mail("tag: inbox");
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(search_mail)
        .expect("Failed to launch Seneca");
}

fn root_widget(mails: &Vector<Mail>) -> impl Widget<MailData> {
    for mail in mails {}
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_flex_child(
            LensWrap::new(
                Scroll::new(
                    List::new(|| {
                        Flex::row()
                            .with_child(
                                Label::new(|mail: &Mail, _env: &Env| mail.date.to_string())
                                    .with_text_color(Color::BLACK)
                                    .fix_width(160.),
                            )
                            .with_default_spacer()
                            .with_child(ClipBox::new(
                                Label::new(|mail: &Mail, _env: &Env| mail.sender.to_string())
                                    .with_text_color(Color::BLACK)
                                    .fix_width(300.),
                            ))
                            .with_default_spacer()
                            .with_flex_child(
                                Label::new(|mail: &Mail, _env: &Env| mail.subject.to_string())
                                    .with_text_color(Color::BLACK),
                                1.0,
                            )
                            .must_fill_main_axis(true)
                            .background(Color::WHITE)
                    })
                    .with_spacing(4.),
                )
                .vertical(),
                lens!(MailData, emails),
            ),
            1.0,
        )
        .background(Color::grey(0.75))
        .padding(10.0)
}
