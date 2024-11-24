use super::{CpuLayout, MemoryLayout};
use crate::state::MonitorWidgets;
use gtk::prelude::*;
use gtk::{Box, Orientation};

pub struct InnerLayout {
    widget: Box,
}

impl InnerLayout {
    pub fn new(monitor_widget: &MonitorWidgets) -> Self {
        let widget = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(15)
            .margin_start(15)
            .margin_end(15)
            .build();

        widget.set_height_request(200);

        let memory_layout = MemoryLayout::new(monitor_widget);
        let cpu_layout = CpuLayout::new(monitor_widget);

        widget.append(cpu_layout.widget());
        widget.append(memory_layout.widget());

        Self { widget }
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
