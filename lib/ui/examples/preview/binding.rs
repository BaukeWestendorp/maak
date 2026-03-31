use gpui::prelude::*;
use gpui::{Keystroke, Window, div};
use maak_ui::{Binding, section};

pub struct BindingPreview {}

impl BindingPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for BindingPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let examples: Vec<(&'static str, &'static str)> = vec![
            ("Copy", "cmd-c"),
            ("Palette", "cmd-shift-p"),
            ("Escape", "escape"),
            ("Enter", "enter"),
            ("Arrow Left", "left"),
            ("Arrow Right", "right"),
            ("Arrow Up", "up"),
            ("Arrow Down", "down"),
            ("Page Up", "pageup"),
            ("Page Down", "pagedown"),
            ("Backspace", "backspace"),
            ("Delete", "delete"),
        ];

        let bindings_row = div().flex().gap_2().flex_wrap().children(
            examples
                .iter()
                .map(|(label, stroke)| {
                    let parsed = Keystroke::parse(stroke)
                        .unwrap_or_else(|_| panic!("invalid keystroke in preview: {stroke}"));

                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(div().w_24().child((*label).to_string()))
                        .child(Binding::new(parsed))
                })
                .collect::<Vec<_>>(),
        );

        div()
            .p_2()
            .size_full()
            .flex()
            .flex_col()
            .gap_2()
            .child(section("Keystrokes", cx).size_full().child(bindings_row))
    }
}
