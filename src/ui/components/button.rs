use gtk::prelude::*;
use gtk::Button as GtkButton;

pub struct Button {
    widget: GtkButton,
}

impl Button {
    pub fn new(label: &str) -> Self {
        let widget = GtkButton::with_label(label);
        Self { widget }
    }

    pub fn widget(&self) -> &GtkButton {
        &self.widget
    }

    pub fn set_hexpand(&self, expand: bool) {
        self.widget.set_hexpand(expand);
    }

    pub fn set_vexpand(&self, expand: bool) {
        self.widget.set_vexpand(expand);
    }
}
