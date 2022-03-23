use druid::{
    im::Vector,
    piet::{Text, TextLayoutBuilder},
    RenderContext,
};
use druid::{Color, Data, Size, Widget};

use crate::mail::{Thread, Email};
use crate::MailData;

impl Widget<MailData> for Email {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut MailData,
        env: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &MailData,
        env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &MailData,
        data: &MailData,
        env: &druid::Env,
    ) {
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &MailData,
        env: &druid::Env,
    ) -> Size {
        let size = Size::new(1150., 1500.); // 1150 is roughly 80 characters of 11pt type
        if bc.is_width_bounded() | bc.is_height_bounded() {
            bc.constrain(size)
        } else {
            size
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &MailData, env: &druid::Env) {
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
