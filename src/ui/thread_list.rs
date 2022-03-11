use std::path::PathBuf;

use crate::{Email, MailData, Thread, BACKGROUND_COLOR, BORDER_COLOR};
use druid::im::vector;
use druid::widget::{prelude::*, Container, FlexParams};
use druid::widget::{ClipBox, CrossAxisAlignment, Flex, Label, List, Padding, Scroll, WidgetExt};
use druid::{Color, Point, Size, WindowConfig, WindowLevel};
use itertools::Itertools;
use mailparse::{dateparse, parse_mail, MailHeaderMap};

pub fn thread_list() -> impl Widget<MailData> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_flex_child(
            Scroll::new(
                List::new(|| {
                    Container::new(
                        Flex::row()
                            .with_flex_child(
                                Flex::column()
                                    .with_child(ClipBox::new(
                                        Label::new(|mail: &Thread, _env: &Env| {
                                            mail.authors.iter().join(", ")
                                        })
                                        .with_text_color(Color::BLACK)
                                        .fix_width(300.),
                                    ))
                                    .with_default_spacer()
                                    .with_child(
                                        Label::new(|mail: &Thread, _env: &Env| {
                                            mail.subject.to_string()
                                        })
                                        .with_text_color(Color::BLACK),
                                    ),
                                0.85,
                            )
                            .with_default_spacer()
                            .with_child(
                                Label::new(|mail: &Thread, _env: &Env| mail.date.to_string())
                                    .with_text_color(Color::BLACK)
                                    .fix_width(160.),
                            )
                            .must_fill_main_axis(true)
                            .background(BACKGROUND_COLOR)
                            .border(BORDER_COLOR, 1.5)
                            .on_click(|ctx, data: &mut Thread, env| {
                                println!("This thread has {} emails.", data.message_paths.len());
                                for mail in data.message_paths.clone() {
                                    println!("Processing {:?}", mail);
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

                                let mail_list = crate::mail_layout();
                                ctx.new_sub_window(
                                    WindowConfig::default()
                                        .window_size(Size::new(700., 600.))
                                        .set_position(Point::new(300., 300.))
                                        .set_level(WindowLevel::AppWindow),
                                    mail_list,
                                    data.clone(),
                                    env.clone(),
                                );
                            }),
                    )
                    .background(BACKGROUND_COLOR)
                    .border(BORDER_COLOR, 0.2)
                    .rounded(3.)
                })
                .with_spacing(4.),
            )
            .vertical()
            .lens(MailData::threads),
            1.0,
        )
        .padding(0.5)
        .border(BORDER_COLOR, 1.)
        .padding(0.5)
}
