use gpui::prelude::*;
use gpui::{Window, div};
use maak_ui::{Button, TitleBar, section};

pub struct TitleBarPreview {}

impl TitleBarPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for TitleBarPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title_bar_no_children = TitleBar::new();
        let title_bar_children = TitleBar::new().child(
            div().flex().gap_2().child("Hello World").child(Button::new("button").child("Button")),
        );

        div()
            .size_full()
            .p_2()
            .flex()
            .gap_2()
            .child(section("Without Children", cx).child(title_bar_no_children))
            .child(section("With Children", cx).child(title_bar_children))
    }
}
