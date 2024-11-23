use gtk::{gdk, CssProvider};

pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("style.css");

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
