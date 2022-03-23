use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use druid::im::{vector, Vector};
use druid::lens::InArc;
use druid::widget::{
    Container, CrossAxisAlignment, Flex, Label, LineBreaking, List, Padding, Scroll,
};
use druid::{ArcStr, Color, Data, Env, Lens, Widget, WidgetExt};
use lazy_static::lazy_static;
use mailparse::{dateparse, parse_mail, MailHeaderMap};
use notmuch::{Database, DatabaseMode};
use regex::Regex;

use crate::{MailData, BORDER_COLOR, THREAD_BACKGROUND_COLOR};

#[derive(Data, Lens, Clone)]
pub struct Email {
    pub body: String,
    pub subject: String,
    pub date: Arc<DateTime<Local>>,
    pub to: String,
    pub cc: Vector<String>,
    pub from: String,
}

#[derive(Clone, Data, Lens)]
pub struct Thread {
    pub authors: Vector<String>,
    pub date: Arc<DateTime<Local>>,
    pub subject: String,
    pub message_paths: Vector<Arc<PathBuf>>,
    pub messages: Vector<Email>,
    pub id: String,
    pub tags: Vector<String>,
    pub viewing: bool,
}

pub fn load_mail(query: ArcStr, event_sink: druid::ExtEventSink, db_location: &OsString) {
    let db = Database::open(Path::new(db_location), DatabaseMode::ReadWrite).unwrap();
    let inbox = db.create_query(&query).unwrap();
    let mut threads = inbox.search_threads().unwrap();
    let mut thread_tracker = Vector::new();

    for thread in threads.by_ref() {
        thread_tracker.push_back(Arc::new(Thread {
            authors: thread.authors().clone().into(),
            date: Arc::new(Local.timestamp(thread.newest_date(), 0)),
            subject: thread.subject().clone().into(),
            message_paths: thread
                .messages()
                .map(|m| Arc::new(m.filename().into()))
                .collect::<Vector<Arc<PathBuf>>>(),
            messages: Vector::new(),
            id: thread.id().into(),
            tags: thread.tags().collect(),
            viewing: false,
        }));
    }
    event_sink.add_idle_callback(|app_data: &mut MailData| {
        app_data.threads = thread_tracker;
        app_data.done_loading = true;
    });
}

pub fn load_thread_from_disk(data: Arc<Thread>) -> Arc<Thread> {
    let mut new_thread = (*data).clone();
    for mail in data.message_paths.clone() {
        let raw = std::fs::read_to_string(&*mail).unwrap_or_default();
        let parsed = parse_mail(raw.as_bytes()).unwrap();
        new_thread.messages.push_back(Email {
            body: if parsed.ctype.mimetype.contains("multipart") {
                let mut body_temp = "Multipart!".to_string();
                for part in parsed.subparts {
                    if part.ctype.mimetype.contains("plain") {
                        body_temp = part.get_body().unwrap_or_default();
                    }
                }
                body_temp
            } else {
                parsed.get_body().unwrap_or_default()
            },
            subject: parsed
                .headers
                .get_first_value("Subject")
                .unwrap_or_default(),
            date: Arc::new(Local.timestamp(
                dateparse(parsed.headers.get_first_value("Date").unwrap().as_str()).unwrap(),
                0,
            )),
            to: parsed.headers.get_first_value("To").unwrap_or_default(),
            from: parsed.headers.get_first_value("From").unwrap_or_default(),
            cc: vector![parsed.headers.get_first_value("Cc").unwrap_or_default()],
        });
    }

    Arc::new(new_thread.to_owned())
}

pub fn mail_layout() -> impl Widget<Arc<Thread>> {
    lazy_static! {
        static ref NAME_REGEX: Regex =
            Regex::new(r"'?(\w+(?:\s+\w+)*)'?\s+<?(\S+@[\w.-]+\.[a-zA-Z]{2,4}\b)").unwrap();
    }
    Scroll::new(
        List::new(|| {
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Padding::new(
                    (5., 0., 0., 5.),
                    Container::new(
                        Flex::column()
                            .cross_axis_alignment(CrossAxisAlignment::Start)
                            .with_child(
                                Label::new(|mail: &Email, _env: &Env| {
                                    let name_text;
                                    if let Some(caps) = NAME_REGEX.captures(&mail.from) {
                                        name_text = match caps.get(1) {
                                            Some(name) => name.as_str(),
                                            None => &mail.from,
                                        };
                                    } else {
                                        name_text = &mail.from;
                                    }

                                    format!("{}", name_text)
                                })
                                .with_text_color(Color::BLACK)
                                .with_font(crate::UI_FONT),
                            )
                            .with_child(
                                Label::new(|mail: &Email, _env: &Env| format!("{}", mail.subject))
                                    .with_text_color(Color::BLACK)
                                    .with_font(crate::UI_FONT)
                                    .with_text_size(18.),
                            )
                            .with_child(
                                Label::new(|mail: &Email, _env: &Env| {
                                    format!("{}", mail.date.format("%Y-%m-%d %H:%M:%S").to_string())
                                })
                                .with_text_color(Color::BLACK)
                                .with_font(crate::UI_FONT_LIGHT),
                            ),
                    )
                    .expand_width()
                    .background(THREAD_BACKGROUND_COLOR)
                    .border(BORDER_COLOR, 0.5),
                ))
                .with_child(Padding::new(
                    (5., 0., 0., 0.),
                    Container::new(
                        Scroll::new(
                            Label::new(|mail: &Email, _env: &Env| {
                                format!("{}", august::convert(&mail.body, 80))
                            })
                            .with_line_break_mode(LineBreaking::WordWrap),
                        )
                        .vertical(),
                    )
                    .expand_width()
                    .background(THREAD_BACKGROUND_COLOR)
                    .border(BORDER_COLOR, 1.5)
                    .rounded(2.),
                ))
        })
        .lens(InArc::new(Thread::messages)),
    )
    .vertical()
}
