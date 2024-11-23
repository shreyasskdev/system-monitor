use crate::state::MonitorWidgets;
use gtk::{prelude::*, Box, Label, Orientation, ProgressBar};

pub struct MemoryLayout {
    widget: Box,
    // progress: ProgressBar,
}

impl MemoryLayout {
    pub fn new(monitor_widget: &MonitorWidgets) -> Self {
        let widget = Box::builder().orientation(Orientation::Vertical).build();

        let label = Label::new(Some("Memory"));
        label.set_css_classes(&["section_head"]);
        widget.append(&label);

        let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
        widget.append(&separator);

        let memory = Box::builder().orientation(Orientation::Horizontal).build();
        let label2 = Label::new(Some("Memory usage"));

        let progress = &monitor_widget.memory_progress;
        progress.set_hexpand(true);
        // progress.shows_text();
        let label3 = &monitor_widget.memory_label;

        memory.append(&label2);
        memory.append(progress);
        memory.append(label3);
        memory.set_hexpand(true);
        widget.append(&memory);

        widget.set_hexpand(true);
        widget.set_css_classes(&["box"]);
        Self { widget }
    }

    // pub fn set_progress(&self, value: f64) {
    //     self.progress.set_fraction(value);
    // }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
