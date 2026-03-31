use gpui::prelude::*;
use gpui::{Window, div};
use maak_ui::{Icon, IconSize, IconVariant, section};

pub struct IconPreview {}

impl IconPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for IconPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().p_2().size_full().flex().flex_col().gap_2().child(
            section("Sizes", cx).w_full().h_1_4().child(
                div()
                    .flex()
                    .gap_2()
                    .flex_wrap()
                    .child(Icon::new(IconVariant::Album, IconSize::ExtraSmall))
                    .child(Icon::new(IconVariant::Album, IconSize::Small))
                    .child(Icon::new(IconVariant::Album, IconSize::Regular))
                    .child(Icon::new(IconVariant::Album, IconSize::Large)),
            ),
        )
    }
}
