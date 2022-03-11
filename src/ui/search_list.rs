use std::sync::Arc;

use druid::widget::{Container, Label, List, Painter, Widget};
use druid::{lens, ArcStr, Color, Data, Lens, LensExt, RenderContext, WidgetExt};

use crate::{MailData, Searches, SEARCH_BACKGROUND_COLOR, SEARCH_CHANGE};

pub fn search_sidebar() -> impl Widget<MailData> {
    let search_lens = (
        lens!(MailData, searches).then(lens!(Searches, selected)),
        lens!(MailData, searches).then(lens!(Searches, search_list)),
    );
    Container::new(
        List::new(|| {
            Container::new(Label::dynamic(|data: &(ArcStr, (ArcStr, ArcStr)), _| data.1 .0.to_string()).with_text_size(18.).center()
                .on_click(|ctx, data: &mut (ArcStr, (ArcStr, ArcStr)), _env| {
                    data.0 = data.1 .1.clone();
                    ctx.submit_command(SEARCH_CHANGE.with(data.0.clone()));
                })
                .background(Painter::new(|ctx, data: &(ArcStr, (ArcStr, ArcStr)), env| {
                    let bounds = ctx.size().to_rect();
                    if data.1 .1.same(&data.0) {
                        ctx.fill(bounds, &Color::BLUE);
                    } else {
                        ctx.fill(bounds, &env.get(SEARCH_BACKGROUND_COLOR));
                    }
                }))).rounded(8.)
        })
        .lens(search_lens),
    )
    .background(SEARCH_BACKGROUND_COLOR)
}
