use super::{InnerLayout, MemoryLayout};
use crate::ui::components::Button;
use gtk::prelude::*;
use gtk::{Box, Orientation};

pub struct MainLayout {
    widget: Box,
}

impl MainLayout {
    pub fn new() -> Self {
        let widget = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(15)
            .margin_top(15)
            .margin_bottom(15)
            .margin_start(15)
            .margin_end(15)
            .build();

        let inner_layout = InnerLayout::new();
        let button1 = Button::new("Button 1");
        button1.set_vexpand(true);

        widget.append(inner_layout.widget());
        widget.append(button1.widget());

        Self { widget }
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
