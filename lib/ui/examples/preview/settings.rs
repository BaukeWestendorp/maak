use gpui::prelude::*;
use gpui::{EmptyView, Window, div};
use maak_ui::{Button, SettingsAppExt as _, section};

pub struct SettingsPreview {}

impl SettingsPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for SettingsPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().p_2().size_full().flex().flex_col().gap_2().child(
            section("Settings window", cx).size_full().child(
                div()
                    .flex()
                    .gap_2()
                    .flex_wrap()
                    .child(Button::new("open-settings").child("Open Settings").on_click(
                        |_, _window, cx| {
                            cx.open_settings(None, |_window, cx| cx.new(|_| EmptyView).into());
                        },
                    ))
                    .child(Button::new("close-settings").child("Close Settings").on_click(
                        |_, _window, cx| {
                            cx.close_settings();
                        },
                    )),
            ),
        )
    }
}
