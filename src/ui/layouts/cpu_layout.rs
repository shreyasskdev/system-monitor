use crate::state::MonitorWidgets;
use gtk::{prelude::*, Box, Label, Orientation};

pub struct CpuLayout {
    widget: Box,
}

impl CpuLayout {
    pub fn new(monitor_widget: &MonitorWidgets) -> Self {
        let widget = Box::builder().orientation(Orientation::Vertical).build();

        let label = Label::new(Some("Cpu"));
        label.set_css_classes(&["section_head"]);
        widget.append(&label);

        let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
        widget.append(&separator);

        widget.set_hexpand(true);
        widget.set_css_classes(&["box"]);
        Self { widget }
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
