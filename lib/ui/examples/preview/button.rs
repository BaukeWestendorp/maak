use gpui::prelude::*;
use gpui::{Window, div};
use maak_ui::{Button, Icon, IconSize, IconVariant, section};

pub struct ButtonPreview {}

impl ButtonPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for ButtonPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_2()
            .size_full()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                section("States", cx).size_full().child(
                    div()
                        .flex()
                        .gap_2()
                        .flex_wrap()
                        .child(Button::new("default").child("Default"))
                        .child(Button::new("selected").selected(true).child("Selected"))
                        .child(Button::new("disabled").disabled(true).child("Disabled"))
                        .child(
                            Button::new("icon")
                                .icon(Icon::new(IconVariant::Plus, IconSize::ExtraSmall))
                                .child("With Icon"),
                        ),
                ),
            )
            .child(
                section("Click handler", cx).size_full().child(
                    div()
                        .flex()
                        .gap_2()
                        .flex_wrap()
                        .child(Button::new("click-me").child("Click Me").on_click(|_, _, _| {
                            log::info!("button clicked");
                        }))
                        .child(
                            Button::new("click-me-too")
                                .icon(Icon::new(IconVariant::ArrowRight, IconSize::Small))
                                .child("Click Me Too")
                                .on_click(|_, _, _| {
                                    log::info!("button clicked (with icon)");
                                }),
                        ),
                ),
            )
    }
}
