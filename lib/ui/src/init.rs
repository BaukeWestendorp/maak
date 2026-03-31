pub fn init(cx: &mut gpui::App) {
    crate::root::action::init(cx);
    crate::theme::init(cx);
    crate::settings::init(cx);
    crate::table::action::init(cx);
}

pub mod simple {
    use gpui::prelude::*;
    use gpui::{
        App, Entity, FocusHandle, FontWeight, Menu, MenuItem, Pixels, QuitMode, SharedString, Size,
        TitlebarOptions, Window, WindowBounds, WindowOptions, div, px, size,
    };

    use crate::{ActiveTheme, Root, TitleBar, h_flex};

    pub mod action {
        gpui::actions!([Quit]);
        pub(crate) fn init(cx: &mut gpui::App) {
            cx.bind_keys([gpui::KeyBinding::new("secondary-q", Quit, None)]);
            cx.on_action::<Quit>(|_, cx| cx.quit());
        }
    }

    pub fn build_simple_app() -> SimpleAppBuilder {
        SimpleAppBuilder::new()
    }

    pub struct SimpleAppBuilder {
        window_title: SharedString,
        window_size: Size<Pixels>,
        activate: bool,

        #[cfg(feature = "config")]
        config: Option<config::Config>,
    }

    impl Default for SimpleAppBuilder {
        fn default() -> Self {
            Self {
                window_title: "Preview App".into(),
                window_size: size(px(1080.0), px(720.0)),
                activate: true,

                #[cfg(feature = "config")]
                config: None,
            }
        }
    }

    impl SimpleAppBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn window_title(mut self, window_title: impl Into<SharedString>) -> Self {
            self.window_title = window_title.into();
            self
        }

        pub fn window_size(mut self, window_size: Size<Pixels>) -> Self {
            self.window_size = window_size;
            self
        }

        pub fn activate(mut self, activate: bool) -> Self {
            self.activate = activate;
            self
        }

        #[cfg(feature = "config")]
        pub fn config(mut self, config: config::Config) -> Self {
            self.config = config.into();
            self
        }

        pub fn run<V>(
            self,
            build_content: impl FnOnce(&mut Window, &mut App) -> Entity<V> + 'static,
        ) where
            V: 'static + Render,
        {
            gpui_platform::application()
                .with_assets(crate::Assets::default())
                .with_quit_mode(QuitMode::LastWindowClosed)
                .run(move |cx: &mut App| {
                    crate::init(cx);

                    #[cfg(feature = "config")]
                    if let Some(config) = self.config {
                        crate::feature::config::init(config, cx);
                    }

                    action::init(cx);
                    cx.set_menus([Menu::new("").items([MenuItem::action("Quit", action::Quit)])]);

                    if self.activate {
                        cx.activate(true);
                    }

                    cx.open_window(
                        WindowOptions {
                            titlebar: Some(TitlebarOptions {
                                title: Some(self.window_title),
                                appears_transparent: true,
                                ..Default::default()
                            }),
                            window_bounds: Some(WindowBounds::centered(self.window_size, cx)),
                            ..Default::default()
                        },
                        |window, cx| {
                            let content = (build_content)(window, cx);
                            let view = cx.new(|cx| SimpleAppView::new(content, cx));
                            cx.new(|cx| Root::new(view, window, cx))
                        },
                    )
                    .unwrap();
                });
        }
    }

    struct SimpleAppView<V: Render + 'static> {
        content: Entity<V>,
        focus_handle: FocusHandle,
    }

    impl<V: Render + 'static> SimpleAppView<V> {
        fn new(content: Entity<V>, cx: &mut Context<Self>) -> Self {
            Self { content, focus_handle: cx.focus_handle() }
        }

        fn render_title_bar_content(
            &mut self,
            window: &mut Window,
            cx: &mut Context<Self>,
        ) -> impl IntoElement {
            h_flex().size_full().justify_between().child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_color(cx.theme().fg_secondary)
                    .child(window.window_title()),
            )
        }

        fn render_content(
            &mut self,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> impl IntoElement {
            self.content.clone()
        }
    }

    impl<V: Render + 'static> Render for SimpleAppView<V> {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .track_focus(&self.focus_handle)
                .flex()
                .flex_col()
                .size_full()
                .child(TitleBar::new().child(self.render_title_bar_content(window, cx)))
                .child(div().size_full().overflow_hidden().child(self.render_content(window, cx)))
        }
    }
}
