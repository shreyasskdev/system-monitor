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

use std::cell::RefCell;
use std::rc::Rc;

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
                // monitor_widget
                //     .memory_progress
                //     .set_fraction(state.memory_usage);
                update_progress(&monitor_widget.memory_progress, state.memory_usage);
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

fn update_progress(progress_bar: &gtk::ProgressBar, target: f64) {
    let start_value = progress_bar.fraction();
    let total_distance = target - start_value;

    let start_time = Rc::new(RefCell::new(None::<std::time::Instant>));
    let progress_bar = progress_bar.clone();

    const ANIMATION_DURATION_MS: u64 = 300;

    glib::timeout_add_local(Duration::from_millis(16), move || {
        let mut start_time = start_time.borrow_mut();

        if start_time.is_none() {
            *start_time = Some(std::time::Instant::now());
        }

        let elapsed = start_time.unwrap().elapsed();
        let progress = (elapsed.as_millis() as f64 / ANIMATION_DURATION_MS as f64).min(1.0);

        let cubic_progress = 1.0 - (1.0 - progress).powi(3);
        let current = start_value + (total_distance * cubic_progress);

        progress_bar.set_fraction(current);

        if progress >= 1.0 {
            progress_bar.set_fraction(target);
            glib::ControlFlow::Break
        } else {
            glib::ControlFlow::Continue
        }
    });
}
