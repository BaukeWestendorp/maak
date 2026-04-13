use gpui::prelude::*;
use gpui::{App, Div, Styled, div};

use crate::ActiveTheme;

pub fn todo(cx: &App) -> Div {
    div()
        .size_full()
        .border_1()
        .border_color(cx.theme().warning)
        .bg(cx.theme().warning.opacity(0.2))
        .text_color(cx.theme().warning)
        .flex()
        .justify_center()
        .items_center()
        .child("TODO")
}
