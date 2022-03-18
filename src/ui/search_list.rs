use druid::widget::{Container, Label, List, Padding, Painter, Widget};
use druid::{
    lens, ArcStr, Data, Insets, LensExt, LinearGradient, RenderContext, UnitPoint,
    WidgetExt,
};

use crate::{MailData, Searches, SEARCH_BACKGROUND_COLOR, SEARCH_CHANGE, SEARCH_SELECTED_COLOR};

pub fn search_sidebar() -> impl Widget<MailData> {
    let search_lens = (
        lens!(MailData, searches).then(lens!(Searches, selected)),
        lens!(MailData, searches).then(lens!(Searches, search_list)),
    );
    Container::new(
        List::new(|| {
            Container::new(
                Padding::new(
                    Insets::new(6., 0., 0., 0.),
                    Label::dynamic(|data: &(ArcStr, (ArcStr, ArcStr)), _| data.1 .0.to_string())
                        .with_font(crate::UI_FONT_LARGE)
                        .on_click(|ctx, data: &mut (ArcStr, (ArcStr, ArcStr)), _env| {
                            data.0 = data.1 .1.clone();
                            ctx.submit_command(SEARCH_CHANGE.with(data.0.clone()));
                        }),
                )
                .background(Painter::new(
                    |ctx, data: &(ArcStr, (ArcStr, ArcStr)), env| {
                        let bounds = ctx.size().to_rect();
                        if data.1 .1.same(&data.0) {
                            ctx.fill(
                                bounds,
                                &LinearGradient::new(
                                    UnitPoint::LEFT,
                                    UnitPoint::RIGHT,
                                    (
                                        env.get(SEARCH_SELECTED_COLOR),
                                        env.get(SEARCH_BACKGROUND_COLOR),
                                    ),
                                ),
                            );
                        } else {
                            ctx.fill(bounds, &env.get(SEARCH_BACKGROUND_COLOR));
                        }
                    },
                ))
                .rounded(5.),
            )
            .expand_width()
            .padding(Insets::new(12., 6., 12., 6.))
        })
        .lens(search_lens),
    )
    .padding(Insets::new(12., 12., 0., 12.))
    .background(SEARCH_BACKGROUND_COLOR)
    .align_left()
}
