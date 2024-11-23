use gtk::prelude::*;
use gtk::{Box, Orientation};

pub struct MemoryLayout {
    widget: Box,
}

impl MemoryLayout {
    pub fn new() -> Self {
        let widget = Box::builder().orientation(Orientation::Vertical).build();

        widget.set_hexpand(true);
        widget.set_css_classes(&["box"]);

        Self { widget }
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
