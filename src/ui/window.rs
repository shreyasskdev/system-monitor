use super::layouts::MainLayout;
use crate::styles;
use gtk::{prelude::WidgetExt, Application, ApplicationWindow};

use crate::state::{MonitorWidgets, SystemMetrics};

pub struct MainWindow {
    window: ApplicationWindow,
}

impl MainWindow {
    pub fn new(app: &Application, monitor_widget: &MonitorWidgets) -> Self {
        styles::load_css();

        let main_layout = MainLayout::new(monitor_widget);

        let window = ApplicationWindow::builder()
            .title("System Monitor")
            .application(app)
            .child(main_layout.widget())
            .build();

        window.show();

        Self { window }
    }
}
