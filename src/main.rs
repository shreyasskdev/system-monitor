mod state;
mod styles;
mod ui;

use gtk::prelude::*;
use gtk::{Application, Label, ProgressBar};

use std::sync::Arc;
use std::sync::Mutex;

use glib;
use std::thread;
use std::time::Duration;
use sysinfo::System;

use crate::state::{MonitorWidgets, SystemMetrics};

fn main() {
    let state = Arc::new(Mutex::new(SystemMetrics {
        cpu_usage: 0.75,
        memory_usage: 0.50,
    }));

    let monitor_clone = Arc::clone(&state);
    thread::spawn(move || {
        monitor_system(monitor_clone);
    });

    let app = Application::builder()
        .application_id("com.system.monitor")
        .build();

    app.connect_activate(move |app| {
        let monitor_widget = MonitorWidgets {
            cpu_progress: ProgressBar::new(),
            cpu_label: Label::new(Some("")),
            memory_progress: ProgressBar::new(),
            memory_label: Label::new(Some("")),
        };

        ui::MainWindow::new(app, &monitor_widget);

        // Update UI on main thread
        let state_clone = Arc::clone(&state);
        glib::timeout_add_local(Duration::from_millis(1000), move || {
            if let Ok(state) = state_clone.lock() {
                monitor_widget
                    .memory_progress
                    .set_fraction(state.memory_usage);
                monitor_widget
                    .memory_label
                    .set_text(format!("{:.2}%", (state.memory_usage * 100.0)).as_str());
            }
            glib::ControlFlow::Continue
        });
    });
    app.run();
}

fn monitor_system(main_window: Arc<Mutex<SystemMetrics>>) {
    let mut sys = System::new_all();
    // let mut count = 0;
    loop {
        sys.refresh_memory();

        let total_memory = sys.total_memory() as f64;
        let used_memory = sys.used_memory() as f64;

        let memory_usage = used_memory / total_memory;

        if let Ok(mut widgets) = main_window.lock() {
            widgets.memory_usage = memory_usage;
        }

        thread::sleep(Duration::new(1, 0));
    }
}
