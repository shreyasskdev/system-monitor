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
            .build();

        widget.set_height_request(250);

        let memory_layout = MemoryLayout::new(monitor_widget);
        let cpu_layout = CpuLayout::new(monitor_widget);

        // monitor_widget.memory_progress.set_fraction(0.75);

        widget.append(cpu_layout.widget());
        widget.append(memory_layout.widget());

        Self { widget }
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
