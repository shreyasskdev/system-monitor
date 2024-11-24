use super::InnerLayout;
use crate::state::MonitorWidgets;
// use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Adjustment, Box, Orientation, ScrolledWindow};

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

        // Create the list store - removed the type parameter from new()
        // Create a typed ListStore for StringObject
        let store = gio::ListStore::with_type(gtk::StringObject::static_type());

        // Populate the store with data
        let data = vec![
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
            ["1", "sdnv"],
        ];
        for [id, value] in data {
            let text = format!("{}: {}", id, value);
            store.append(&gtk::StringObject::new(&text));
        }

        // Create factory for list items
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let label = gtk::Label::new(None);
            label.set_xalign(0.0);
            label.set_margin_start(12);
            label.set_margin_end(12);
            label.set_margin_top(6);
            label.set_margin_bottom(6);
            list_item.set_child(Some(&label));
        });

        factory.connect_bind(move |_, list_item| {
            let string_object = list_item
                .item()
                .and_downcast::<gtk::StringObject>()
                .expect("The item has to be a StringObject");

            let label = list_item
                .child()
                .and_downcast::<gtk::Label>()
                .expect("The child has to be a Label");

            label.set_label(&string_object.string());
        });

        // Create the ListView
        let selection_model = gtk::NoSelection::new(Some(store));
        let list_view = gtk::ListView::new(Some(selection_model), Some(factory));
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

        widget.append(inner_layout.widget());
        widget.append(&scrolled_window);

        Self { widget }
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
