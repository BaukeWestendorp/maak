use gpui::prelude::*;
use gpui::{Window, div, point, px};
use maak_ui::{ActiveTheme as _, dot_grid, line_grid, scrollable_line_grid, section};

pub struct GridPreview {}

impl GridPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for GridPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_2()
            .size_full()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                section("Dot grid", cx)
                    .w_full()
                    .h_48()
                    .overflow_hidden()
                    .child(dot_grid(px(12.0), cx.theme().accent.opacity(0.75)).size_full()),
            )
            .child(
                section("Line grid", cx)
                    .w_full()
                    .h_48()
                    .overflow_hidden()
                    .child(line_grid(px(16.0), cx.theme().accent.opacity(0.75)).size_full()),
            )
            .child(
                section("Scrollable grid", cx).w_full().h_48().child(
                    div().size_full().relative().overflow_hidden().child(
                        scrollable_line_grid(
                            &point(px(3.0), px(3.0)),
                            px(16.0),
                            cx.theme().accent.opacity(0.75),
                        )
                        .size_full(),
                    ),
                ),
            )
    }
}
