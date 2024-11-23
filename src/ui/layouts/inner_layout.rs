use super::MemoryLayout;
use crate::ui::components::Button;
use gtk::prelude::*;
use gtk::{Box, Orientation};

pub struct InnerLayout {
    widget: Box,
}

impl InnerLayout {
    pub fn new() -> Self {
        let widget = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(15)
            .build();

        widget.set_height_request(250);

        let button2 = Button::new("Button 2");
        button2.set_hexpand(true);

        let memory_layout = MemoryLayout::new();

        widget.append(button2.widget());
        widget.append(memory_layout.widget());

        Self { widget }
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
