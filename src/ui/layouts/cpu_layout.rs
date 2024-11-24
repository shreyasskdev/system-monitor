use crate::state::MonitorWidgets;
use gtk::{prelude::*, Box, Grid, Label, Orientation, Separator};

pub struct CpuLayout {
    widget: Grid,
}

impl CpuLayout {
    pub fn new(monitor_widget: &MonitorWidgets) -> Self {
        let widget = Grid::new();
        widget.set_hexpand(true);
        widget.set_css_classes(&["box"]);

        // heading
        let heading = Label::new(Some("Cpu"));
        heading.set_css_classes(&["section_head"]);
        let separator = Separator::new(Orientation::Horizontal);

        // layouts
        let layout = Box::builder().orientation(Orientation::Horizontal).build();
        let inner_layout1 = Box::builder().orientation(Orientation::Vertical).build();
        let inner_layout2 = Box::builder().orientation(Orientation::Vertical).build();
        layout.append(&inner_layout1);
        layout.append(&inner_layout2);

        // appending progress bars
        let half = monitor_widget.cpu_progress.len() / 2;
        monitor_widget
            .cpu_progress
            .iter()
            .enumerate()
            .for_each(|(index, progress_bar)| {
                let label = format!("CPU {}", index + 1);
                progress_bar.set_hexpand(true);

                let label_widget = Label::new(Some(&label));
                label_widget.set_size_request(50, -1);

                if index < half {
                    widget.attach(&label_widget, 0, 2 + index as i32, 1, 1);
                    widget.attach(progress_bar, 1, 2 + index as i32, 1, 1);
                } else {
                    widget.attach(&label_widget, 3, 2 + (index - half) as i32, 1, 1);
                    widget.attach(progress_bar, 4, 2 + (index - half) as i32, 1, 1);
                }
            });

        // appending main layout
        widget.attach(&heading, 0, 0, 5, 1);
        widget.attach(&separator, 0, 1, 5, 1);

        Self { widget }
    }

    pub fn widget(&self) -> &Grid {
        &self.widget
    }
}
