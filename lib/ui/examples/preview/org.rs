use gpui::prelude::*;
use gpui::{Window, div};
use maak_ui::{ActiveTheme as _, section};

pub struct OrgPreview {}

impl OrgPreview {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for OrgPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let section_child = div()
            .size_24()
            .border_1()
            .border_color(cx.theme().accent)
            .bg(cx.theme().accent.opacity(0.2))
            .child(div().size_full().flex().justify_center().items_center().child("Content"));

        div()
            .p_2()
            .size_full()
            .flex()
            .gap_2()
            .child(
                section("Section With Child", cx)
                    .child(section("Section Title", cx).w_48().child(section_child)),
            )
            .child(section("Section Without Child", cx))
    }
}
