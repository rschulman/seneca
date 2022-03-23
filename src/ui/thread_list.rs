use std::sync::Arc;
use std::time::Duration;

use crate::{ui::virt_list::VirtList, MailData, Thread};
use crate::{LOAD_THREAD, MARK_READ, THREAD_BACKGROUND_COLOR, THREAD_SELECTED_COLOR};
use chrono::Local;
use druid::kurbo::Circle;
use druid::widget::{CrossAxisAlignment, Flex, Label, LineBreaking, WidgetExt};
use druid::{
    lens, Color, Env, Event, LifeCycle, MouseButton, Point, Rect, RenderContext, Size,
    TextAlignment, TimerToken, Widget,
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

impl Widget<(Option<Arc<Thread>>, Arc<Thread>)> for ThreadWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut (Option<Arc<Thread>>, Arc<Thread>),
        _env: &Env,
    ) {
        match event {
            Event::Timer(id) => {
                // Check if the timer in question is for us, and that this thread is still being viewed
                if *id == self.timer_id
                    && data.0.is_some()
                    && data.0.as_ref().unwrap().id == data.1.id
                {
                    // If so, remove the "unread" tag
                    ctx.submit_command(MARK_READ.with(data.1.clone()));
                    let mut new_thread = (*data.1).clone().to_owned();
                    new_thread.tags.retain(|item| item != &"unread".to_string());
                    data.1 = Arc::new(new_thread);
                    ctx.request_paint();
                }
            }
            Event::MouseUp(evt) => {
                if evt.button == MouseButton::Left {
                    ctx.submit_command(LOAD_THREAD.with(data.1.clone()));
                    self.timer_id = ctx.request_timer(Duration::from_secs(2));
                    ctx.request_paint();
                }
            }
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &(Option<Arc<Thread>>, Arc<Thread>),
        env: &Env,
    ) {
        match event {
            LifeCycle::WidgetAdded => {
                self.senders = Some(
                    Label::new(|mail: &Thread, _env: &Env| mail.authors.iter().join(", "))
                        .with_text_color(Color::BLACK)
                        .with_font(crate::UI_FONT)
                        .with_text_alignment(TextAlignment::Start)
                        .with_line_break_mode(LineBreaking::Clip),
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
                    .lifecycle(ctx, event, &data.1, env);
                self.subject
                    .as_mut()
                    .unwrap()
                    .lifecycle(ctx, event, &data.1, env);
                self.date
                    .as_mut()
                    .unwrap()
                    .lifecycle(ctx, event, &data.1, env);
            }
            _ => {}
        }
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &(Option<Arc<Thread>>, Arc<Thread>),
        data: &(Option<Arc<Thread>>, Arc<Thread>),
        env: &Env,
    ) {
        self.senders
            .as_mut()
            .unwrap()
            .update(ctx, &old_data.1, &data.1, env);
        self.subject
            .as_mut()
            .unwrap()
            .update(ctx, &old_data.1, &data.1, env);
        self.date
            .as_mut()
            .unwrap()
            .update(ctx, &old_data.1, &data.1, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &(Option<Arc<Thread>>, Arc<Thread>),
        env: &Env,
    ) -> druid::Size {
        self.date_size = self.date.as_mut().unwrap().layout(ctx, bc, &data.1, env);
        let senders_bc = druid::BoxConstraints::new(
            bc.min().clone(),
            Size::new(bc.max().width - self.date_size.width, bc.max().height),
        ); // Clip the senders if they're going to overlap the sent date
        self.senders
            .as_mut()
            .unwrap()
            .layout(ctx, &senders_bc, &data.1, env);
        self.subject.as_mut().unwrap().layout(ctx, bc, &data.1, env);
        druid::Size::new(bc.max().width, THREAD_HEIGHT)
    }

    fn paint(
        &mut self,
        ctx: &mut druid::PaintCtx,
        data: &(Option<Arc<Thread>>, Arc<Thread>),
        env: &Env,
    ) {
        let size = ctx.size();
        let rect = size.to_rect();
        let bg_color = if data.0.is_some() && data.0.as_ref().unwrap().id == data.1.id {
            &THREAD_SELECTED_COLOR
        } else {
            &THREAD_BACKGROUND_COLOR
        };
        ctx.fill(rect, &env.get(bg_color));

        let radius = THREAD_HEIGHT * 0.1;
        if data.1.tags.contains(&"unread".to_string()) {
            ctx.fill(
                Circle::new(Point::new(14., size.height * 0.5), radius),
                &Color::rgb8(20, 217, 235),
            );
        }

        self.senders
            .as_ref()
            .unwrap()
            .draw_at(ctx, Point::new(radius * 4.3, size.height * 0.2));

        self.subject
            .as_ref()
            .unwrap()
            .draw_at(ctx, Point::new(radius * 4.3, size.height * 0.5));

        ctx.fill(
            Rect::new(
                size.width - self.date_size.width,
                size.height * 0.2,
                size.width,
                (size.height * 0.2) + 15.,
            ),
            &env.get(bg_color),
        );

        self.date.as_ref().unwrap().draw_at(
            ctx,
            Point::new(size.width - self.date_size.width, size.height * 0.2),
        );
    }
}

pub fn thread_list() -> impl Widget<MailData> {
    let widget_lens = (lens!(MailData, loaded_thread), lens!(MailData, threads));
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_flex_child(
            VirtList::vertical(THREAD_HEIGHT, || ThreadWidget::new()).lens(widget_lens),
            1.0,
        )
        .padding(0.5)
}
