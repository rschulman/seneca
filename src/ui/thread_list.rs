use crate::{Email, MailData, Thread, ui::virt_list::VirtList, BACKGROUND_COLOR};
use chrono::Local;
use druid::im::vector;
use druid::widget::{prelude::*, Container};
use druid::widget::{CrossAxisAlignment, Flex, Label, WidgetExt};
use druid::{Color, Insets, Point, Size, TextAlignment, WindowConfig, WindowLevel};
use itertools::Itertools;
use mailparse::{dateparse, parse_mail, MailHeaderMap};

pub fn thread_list() -> impl Widget<MailData> {
    const THREAD_HEIGHT: f64 = 70.0;
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_flex_child(
                VirtList::vertical(THREAD_HEIGHT, || {
                    Container::new(
                        Flex::row()
                            .cross_axis_alignment(CrossAxisAlignment::Start)
                            .with_flex_child(Container::new(
                                Flex::column()
                                    .cross_axis_alignment(CrossAxisAlignment::Start)
                                    .with_child(
                                        Label::new(|mail: &Thread, _env: &Env| {
                                            mail.authors.iter().join(", ")
                                        })
                                        .with_text_color(Color::BLACK)
                                        .with_font(crate::UI_FONT)
                                        .with_text_alignment(TextAlignment::Start)
                                        .padding(Insets::new(10., 10., 10., 0.))
                                    )
                                    .with_default_spacer()
                                    .with_child(
                                        Label::new(|mail: &Thread, _env: &Env| {
                                            mail.subject.to_string()
                                        })
                                        .with_text_color(Color::BLACK)
                                        .with_text_alignment(TextAlignment::Start)
                                        .with_font(crate::UI_FONT_LIGHT)
                                        .padding(Insets::new(10., 0., 10., 10.)),
                                    )).fix_width(300.),
                                0.85,
                            )
                            .with_default_spacer()
                            .with_child(
                                Label::new(|mail: &Thread, _env: &Env| {
                                    let hours_ago = Local::now().signed_duration_since(*mail.date).num_hours();
                                    if hours_ago < 24 {
                                        format!("{} hours ago", hours_ago)
                                    } else {
                                        format!("{}", mail.date.format("%B %e, %Y"))
                                    }
                                    })
                                    .with_text_color(Color::BLACK)
                                    .with_font(crate::UI_FONT_LIGHT)
                                    .with_text_size(11.0)
                                    .with_text_alignment(TextAlignment::Start)
                                        .padding(Insets::new(10., 10., 10., 10.)),
                            )
                            .must_fill_main_axis(true)
                            .background(BACKGROUND_COLOR)
                            .on_click(|ctx, data: &mut Thread, env| spawn_thread_window(ctx, data, env)
                            ),
                    )
                    .background(BACKGROUND_COLOR)
                    .fix_height(THREAD_HEIGHT)
                })
            .lens(MailData::threads),
            1.0,
        )
        .padding(0.5)
}

fn spawn_thread_window(ctx: &mut EventCtx, data: &mut Thread, env: &Env) {
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

}