use crate::{ui::virt_list::VirtList, MailData, Thread};
use crate::{LOAD_THREAD, THREAD_BACKGROUND_COLOR};
use chrono::Local;
use druid::kurbo::Circle;
use druid::widget::{prelude::*, Container, Painter};
use druid::widget::{CrossAxisAlignment, Flex, Label, WidgetExt};
use druid::{Color, Insets, TextAlignment};
use itertools::Itertools;

pub fn thread_list() -> impl Widget<MailData> {
    const THREAD_HEIGHT: f64 = 70.0;
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_flex_child(
            VirtList::vertical(THREAD_HEIGHT, || {
                Container::new(
                    Flex::row()
                        .cross_axis_alignment(CrossAxisAlignment::Start)
                        .with_child(
                            druid::widget::SizedBox::new(Painter::new(
                                |ctx, data: &Thread, _env| {
                                    let bounds = ctx.size().to_rect();
                                    let radius = bounds.height() * 0.1;
                                    if data.tags.contains(&"unread".to_string()) {
                                        ctx.fill(
                                            Circle::new(bounds.center(), radius),
                                            &Color::rgb8(20, 217, 235),
                                        );
                                    }
                                },
                            ))
                            .width(THREAD_HEIGHT * 0.6)
                            .height(THREAD_HEIGHT),
                        )
                        .with_flex_child(
                            Container::new(
                                Flex::column()
                                    .cross_axis_alignment(CrossAxisAlignment::Start)
                                    .with_child(
                                        Label::new(|mail: &Thread, _env: &Env| {
                                            mail.authors.iter().join(", ")
                                        })
                                        .with_text_color(Color::BLACK)
                                        .with_font(crate::UI_FONT)
                                        .with_text_alignment(TextAlignment::Start)
                                        .padding(Insets::new(10., 10., 10., 0.)),
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
                                    ),
                            )
                            .fix_width(300.),
                            0.85,
                        )
                        .with_default_spacer()
                        .with_child(
                            Label::new(|mail: &Thread, _env: &Env| {
                                let hours_ago =
                                    Local::now().signed_duration_since(*mail.date).num_hours();
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
                        .background(THREAD_BACKGROUND_COLOR)
                        .on_click(|ctx, data: &mut Thread, _env| {
                            ctx.submit_command(LOAD_THREAD.with(data.clone()))
                        }),
                )
                .background(THREAD_BACKGROUND_COLOR)
                .fix_height(THREAD_HEIGHT)
            })
            .lens(MailData::threads),
            1.0,
        )
        .padding(0.5)
}
