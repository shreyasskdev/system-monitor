use crate::state::MonitorWidgets;
use gtk::{prelude::*, Box, Label, Orientation, Separator};

pub struct MemoryLayout {
    widget: Box,
}

impl MemoryLayout {
    pub fn new(monitor_widget: &MonitorWidgets) -> Self {
        let widget = Box::builder().orientation(Orientation::Vertical).build();
        widget.set_css_classes(&["box"]);
        widget.set_hexpand(true);

        // heading
        let heading = Label::new(Some("Memory"));
        heading.set_css_classes(&["section_head"]);
        let separator = gtk::Separator::new(gtk::Orientation::Horizontal);

        // layout 1
        let layout1 = Box::builder().orientation(Orientation::Horizontal).build();
        let label_memory_usage_placeholder = Label::new(Some("Memory usage")); // Memory usage
        let progress = &monitor_widget.memory_progress; // Progress
        progress.set_hexpand(true);
        let label_memory_usage = &monitor_widget.memory_label; // memory percentage

        // layout 2
        let layout2 = Box::builder().orientation(Orientation::Horizontal).build();
        let label_memory_total_placeholder = Label::new(Some("Total")); // Total
        let label_memory_total = Label::new(Some("70GB")); // memory total

        // layout 3
        let layout3 = Box::builder().orientation(Orientation::Horizontal).build();
        let label_memory_used_placeholder = Label::new(Some("Used")); // Used
        let label_memory_used = Label::new(Some("50GB")); // memory used

        // layout 4
        let layout4 = Box::builder().orientation(Orientation::Horizontal).build();
        let label_memory_free_placeholder = Label::new(Some("Free")); // Free
        let label_memory_free = Label::new(Some("20GB")); // memory free

        // appending layout 1
        layout1.append(&label_memory_usage_placeholder);
        layout1.append(progress);
        layout1.append(label_memory_usage);

        // appending layout 2
        layout2.append(&label_memory_total_placeholder);
        layout2.append(&create_spacer());
        layout2.append(&label_memory_total);

        // appending layout 3
        layout3.append(&label_memory_used_placeholder);
        layout3.append(&create_spacer());
        layout3.append(&label_memory_used);

        // appending layout 4
        layout4.append(&label_memory_free_placeholder);
        layout4.append(&create_spacer());
        layout4.append(&label_memory_free);

        // appending main layout
        widget.append(&heading);
        widget.append(&separator);
        widget.append(&layout1);
        widget.append(&layout2);
        widget.append(&layout3);
        widget.append(&layout4);

        Self { widget }
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
fn create_spacer() -> Label {
    let spacer = Label::new(Some(""));
    spacer.set_hexpand(true);
    spacer
}
