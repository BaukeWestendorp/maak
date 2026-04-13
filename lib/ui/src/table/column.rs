use gpui::{Length, SharedString, px, relative};

#[derive(Clone)]
pub struct Column {
    id: SharedString,
    label: SharedString,
    width: Length,
    min_width: Length,
}

impl Column {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            width: relative(1.0).into(),
            min_width: px(100.0).into(),
        }
    }

    pub fn with_min_width(mut self, min_width: Length) -> Self {
        self.min_width = min_width;
        self
    }

    pub fn with_width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn id(&self) -> &SharedString {
        &self.id
    }

    pub fn label(&self) -> &SharedString {
        &self.label
    }

    pub fn width(&self) -> Length {
        self.width
    }

    pub fn min_width(&self) -> Length {
        self.min_width
    }
}
