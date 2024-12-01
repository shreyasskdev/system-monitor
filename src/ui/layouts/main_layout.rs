use super::InnerLayout;
use crate::state::MonitorWidgets;
// use gio::prelude::*;
use gtk::{prelude::*, Label};
use gtk::{Box, Orientation, ScrolledWindow, Separator};

pub struct MainLayout {
    widget: Box,
}

impl MainLayout {
    pub fn new(monitor_widget: &MonitorWidgets) -> Self {
        let widget = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(15)
            .margin_top(15)
            .build();

        let inner_layout = InnerLayout::new(monitor_widget);
        let main_layout = Box::builder()
            .orientation(Orientation::Vertical)
            .css_classes(["box"])
            .build();

        // Heading for main layout
        let head = Box::builder()
            .orientation(Orientation::Horizontal)
            .homogeneous(true)
            .build();
        let pid_label = Label::new(Some("PID"));
        let name_label = Label::new(Some("Name"));
        let usage_label = Label::new(Some("Usage"));
        let memory_label = Label::new(Some("Memory"));
        let separator = Separator::new(Orientation::Horizontal);
        head.append(&pid_label);
        head.append(&name_label);
        head.append(&usage_label);
        head.append(&memory_label);
        main_layout.append(&head);
        main_layout.append(&separator);

        // Create the ListView
        match &monitor_widget.process_details {
            Some(process_details) => {
                let selection_model = gtk::NoSelection::new(Some(process_details.0.clone()));
                let list_view =
                    gtk::ListView::new(Some(selection_model), Some(process_details.1.clone()));

                list_view.set_vexpand(true);

                let scrolled_window = ScrolledWindow::builder()
                    .hscrollbar_policy(gtk::PolicyType::Never)
                    .vscrollbar_policy(gtk::PolicyType::Automatic)
                    .kinetic_scrolling(true)
                    .propagate_natural_height(true)
                    .min_content_height(200)
                    .hexpand(true)
                    .vexpand(true)
                    .child(&list_view)
                    .build();

                main_layout.append(&scrolled_window);

                widget.append(inner_layout.widget());
                widget.append(&main_layout);
            }
            None => {}
        }

        Self { widget }
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
