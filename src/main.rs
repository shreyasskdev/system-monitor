use gtk::gdk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, CssProvider};

fn main() {
    let app = Application::builder()
        .application_id("com.shreyassk.demo")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("style.css");

    gtk::StyleContext::add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let button1 = Button::with_label("Button 1");
    let button2 = Button::with_label("Button 2");
    let button3 = Button::with_label("Button 3");

    let main_layout = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(15)
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();
    let inner_layout = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(15)
        .build();
    let memory_layout = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    load_css();

    inner_layout.set_height_request(250);
    button1.set_vexpand(true);
    button2.set_hexpand(true);
    memory_layout.set_hexpand(true);
    memory_layout.set_css_classes(&["box"]);
    // memory_layout.set_background_color(&RGBA::new(0.3, 0.7, 0.5, 1.0));

    main_layout.append(&inner_layout);
    main_layout.append(&button1);

    inner_layout.append(&button2);
    inner_layout.append(&memory_layout);

    let window = ApplicationWindow::builder()
        .title("UI demo")
        .application(app)
        .child(&main_layout)
        .build();

    window.show();
}
