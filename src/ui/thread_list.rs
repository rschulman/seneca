use crate::{ui::virt_list::VirtList, MailData, Thread};
use crate::{LOAD_THREAD, THREAD_BACKGROUND_COLOR, THREAD_SELECTED_COLOR};
use chrono::Local;
use druid::kurbo::Circle;
use druid::widget::{CrossAxisAlignment, Flex, Label, WidgetExt};
use druid::{
    Color, Env, Event, LifeCycle, Point, RenderContext, Size, TextAlignment, TimerToken,
    Widget,
};
use itertools::Itertools;

const THREAD_HEIGHT: f64 = 60.0;

pub struct ThreadWidget {
    timer_id: TimerToken,
    senders: Option<Label<Thread>>,
    subject: Option<Label<Thread>>,
    date: Option<Label<Thread>>,
    date_size: Size,
}

impl ThreadWidget {
    pub fn new() -> Self {
        Self {
            timer_id: TimerToken::INVALID,
            senders: None,
            subject: None,
            date: None,
            date_size: Size::ZERO,
        }
    }
}

impl Widget<Thread> for ThreadWidget {
    fn event(
        &mut self,
        _ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Thread,
        _env: &Env,
    ) {
        match event {
            Event::Timer(id) => {
                // Check if the timer in question is for us, and that this thread is still being viewed
                if *id == self.timer_id && data.viewing {
                    data.tags.retain(|item| item != &"unread".to_string()); // If so, remove the "unread" tag
                }
            }
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Thread,
        env: &Env,
    ) {
        match event {
            LifeCycle::WidgetAdded => {
                self.senders = Some(
                    Label::new(|mail: &Thread, _env: &Env| mail.authors.iter().join(", "))
                        .with_text_color(Color::BLACK)
                        .with_font(crate::UI_FONT)
                        .with_text_alignment(TextAlignment::Start),
                );
                self.subject = Some(
                    Label::new(|mail: &Thread, _env: &Env| mail.subject.to_string())
                        .with_text_color(Color::BLACK)
                        .with_text_alignment(TextAlignment::Start)
                        .with_font(crate::UI_FONT_LIGHT),
                );
                self.date = Some(
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
                    .with_text_size(11.0),
                );
                self.senders
                    .as_mut()
                    .unwrap()
                    .lifecycle(ctx, event, data, env);
                self.subject
                    .as_mut()
                    .unwrap()
                    .lifecycle(ctx, event, data, env);
                self.date.as_mut().unwrap().lifecycle(ctx, event, data, env);
            }
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &Thread, data: &Thread, env: &Env) {
        println!("New thread!");
        self.senders
            .as_mut()
            .unwrap()
            .update(ctx, old_data, data, env);
        self.subject
            .as_mut()
            .unwrap()
            .update(ctx, old_data, data, env);
        self.date.as_mut().unwrap().update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &Thread,
        env: &Env,
    ) -> druid::Size {
        self.senders.as_mut().unwrap().layout(ctx, bc, data, env);
        self.subject.as_mut().unwrap().layout(ctx, bc, data, env);
        self.date_size = self.date.as_mut().unwrap().layout(ctx, bc, data, env);
        druid::Size::new(bc.max().width, THREAD_HEIGHT)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Thread, env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();
        let bg_color = if data.viewing {
            &THREAD_SELECTED_COLOR
        } else {
            &THREAD_BACKGROUND_COLOR
        };
        ctx.fill(rect, &env.get(bg_color));

        let radius = size.height * 0.1;
        if data.tags.contains(&"unread".to_string()) {
            ctx.fill(
                Circle::new(Point::new(size.width * 0.05, size.height * 0.5), radius),
                &Color::rgb8(20, 217, 235),
            );
        }

        self.senders
            .as_ref()
            .unwrap()
            .draw_at(ctx, Point::new((radius * 2.) + 10., size.height * 0.2));

        self.subject
            .as_ref()
            .unwrap()
            .draw_at(ctx, Point::new((radius * 2.) + 10., size.height * 0.7));

        self.date.as_ref().unwrap().draw_at(
            ctx,
            Point::new(size.width - self.date_size.width, size.height * 0.2),
        );
    }
}

pub fn thread_list() -> impl Widget<MailData> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_flex_child(
            VirtList::vertical(THREAD_HEIGHT, || ThreadWidget::new()).lens(MailData::threads),
            1.0,
        )
        .padding(0.5)
}
