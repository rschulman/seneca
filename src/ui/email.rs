use druid::{
    im::Vector,
    piet::{Text, TextLayoutBuilder},
    RenderContext,
};
use druid::{Color, Data, Size, Widget};
use mailparse::{parse_mail, MailHeaderMap};

#[derive(Clone, Data)]
pub struct Email {
    raw: String,
}

impl Widget<Email> for Email {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Email,
        env: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Email,
        env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &Email,
        data: &Email,
        env: &druid::Env,
    ) {
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &Email,
        env: &druid::Env,
    ) -> Size {
        let size = Size::new(1150., 1500.); // 1150 is roughly 80 characters of 11pt type
        if bc.is_width_bounded() | bc.is_height_bounded() {
            bc.constrain(size)
        } else {
            size
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Email, env: &druid::Env) {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE);

        // Parse the actual text of the email
        let parsed = parse_mail(data.raw.as_bytes()).unwrap();
        let text = ctx.text();
        let layout = text
            .new_text_layout(format!(
                "From: {}",
                parsed.headers.get_first_value("From").unwrap_or("".into())
            ))
            .build()
            .unwrap();
    }
}
