use super::layouts::MainLayout;
use crate::styles;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

pub fn build_ui(app: &Application) {
    styles::load_css();

    let main_layout = MainLayout::new();

    let window = ApplicationWindow::builder()
        .title("UI demo")
        .application(app)
        .child(main_layout.widget())
        .build();

    window.show();
}
