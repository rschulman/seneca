use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use druid::im::{Vector, vector};
use druid::widget::{
    Container, CrossAxisAlignment, Flex, Label, LineBreaking, List, Padding, Scroll,
};
use druid::{ArcStr, Data, Env, Lens, Widget, WidgetExt};
use mailparse::{MailHeaderMap, dateparse, parse_mail};
use notmuch::{Database, DatabaseMode};

use crate::{MailData, THREAD_BACKGROUND_COLOR, BORDER_COLOR};

#[derive(Data, Lens, Clone)]
pub struct Email {
    pub body: String,
    pub subject: String,
    pub date: i64,
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
}

pub fn load_mail(query: ArcStr, event_sink: druid::ExtEventSink, db_location: ArcStr) {
    let db_osstr: OsString = db_location.to_string().into();
    let db = Database::open(Path::new(&db_osstr), DatabaseMode::ReadWrite).unwrap();
    let inbox = db.create_query(&query).unwrap();
    let mut threads = inbox.search_threads().unwrap();
    println!("Loading threads...");
    let mut thread_tracker = Vector::new();

    for thread in threads.by_ref() {
        thread_tracker.push_back(Thread {
            authors: thread.authors().clone().into(),
            date: Arc::new(Local.timestamp(thread.newest_date(), 0)),
            subject: thread.subject().clone().into(),
            message_paths: thread
                .messages()
                .map(|m| Arc::new(m.filename().into()))
                .collect::<Vector<Arc<PathBuf>>>(),
            messages: Vector::new(),
            id: thread.id().into(),
        });
    }
    event_sink.add_idle_callback(|app_data: &mut MailData| {
        println!("Pushing threads...");
        app_data.threads = thread_tracker;
        app_data.done_loading = true;
    });
    println!("Done!");
}

pub fn load_thread_from_disk(data: &mut Thread) {
    for mail in data.message_paths.clone() {
        let raw = std::fs::read_to_string(&*mail).unwrap_or_default();
        let parsed = parse_mail(raw.as_bytes()).unwrap();
        data.messages.push_back(Email {
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
            date: dateparse(
                parsed
                    .headers
                    .get_first_value("Date")
                    .unwrap()
                    .as_str(),
            )
            .unwrap(),
            to: parsed
                .headers
                .get_first_value("To")
                .unwrap_or_default(),
            from: parsed
                .headers
                .get_first_value("From")
                .unwrap_or_default(),
            cc: vector![parsed
                .headers
                .get_first_value("Cc")
                .unwrap_or_default()],
        });
    }
}

pub fn mail_layout() -> impl Widget<Thread> {
    Scroll::new(
        List::new(|| {
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Padding::new(
                    (5., 0., 0., 5.),
                    Container::new(
                        Flex::column()
                            .cross_axis_alignment(CrossAxisAlignment::Start)
                            .with_child(Label::new(|mail: &Email, _env: &Env| {
                                format!("From: {}", mail.from)
                            }))
                            .with_child(Label::new(|mail: &Email, _env: &Env| {
                                format!("Subject: {}", mail.subject)
                            }))
                            .with_child(Label::new(|mail: &Email, _env: &Env| {
                                format!(
                                    "Date: {}",
                                    NaiveDateTime::from_timestamp(mail.date, 0)
                                        .format("%Y-%m-%d %H:%M:%S")
                                        .to_string()
                                )
                            })),
                    )
                    .background(THREAD_BACKGROUND_COLOR)
                    .border(BORDER_COLOR, 1.5)
                    .rounded(2.),
                ))
                .with_child(Padding::new(
                    (5., 0., 0., 0.),
                    Container::new(
                        Scroll::new(
                            Label::new(|mail: &Email, _env: &Env| format!("{}", mail.body))
                                .with_line_break_mode(LineBreaking::WordWrap),
                        )
                        .vertical(),
                    )
                    .background(THREAD_BACKGROUND_COLOR)
                    .border(BORDER_COLOR, 1.5)
                    .rounded(2.),
                ))
        })
        .lens(Thread::messages),
    )
    .vertical()
}
