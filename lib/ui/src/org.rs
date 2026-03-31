use gpui::prelude::*;
use gpui::{App, Div, FontWeight, SharedString, div};

use crate::ActiveTheme as _;

pub fn section(title: impl Into<SharedString>, cx: &App) -> Div {
    let header = div()
        .w_full()
        .px_1()
        .mb_2()
        .font_weight(FontWeight::BOLD)
        .border_b_1()
        .border_color(cx.theme().border_secondary)
        .child(title.into());

    div().flex_col().child(header)
}
