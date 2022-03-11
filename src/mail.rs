use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use chrono::NaiveDateTime;
use druid::im::{vector, Vector};
use druid::platform_menus::mac::file::print;
use druid::{ArcStr, Data, Lens};
use home::home_dir;
use notmuch::{Database, DatabaseMode};

use crate::MailData;

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
    pub date: String,
    pub subject: String,
    pub message_paths: Vector<Arc<PathBuf>>,
    pub messages: Vector<Email>,
    pub id: String,
}

pub fn load_mail(query: ArcStr, event_sink: druid::ExtEventSink) {
    let mut mail_path = home_dir().unwrap();
    mail_path.push("Personal");
    mail_path.push(".mail");
    let db = Database::open(
        &"/home/ross/Personal/.mail/".to_string(),
        DatabaseMode::ReadWrite,
    )
    .unwrap();
    let inbox = db.create_query(&query).unwrap();
    let mut threads = inbox.search_threads().unwrap();
    println!("Loading threads...");
    let mut thread_tracker = Vector::new();

    for thread in threads.by_ref() {
        thread_tracker.push_back(Thread {
            authors: thread.authors().clone().into(),
            date: NaiveDateTime::from_timestamp(thread.newest_date(), 0)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
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
