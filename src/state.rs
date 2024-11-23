pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    // memory_used: u64,
    // memory_total: u64,
    // swap_used: u64,
    // swap_total: u64,
    // disk_used: u64,
    // disk_total: u64,
}

pub struct MonitorWidgets {
    pub cpu_progress: gtk::ProgressBar,
    pub cpu_label: gtk::Label,
    pub memory_progress: gtk::ProgressBar,
    pub memory_label: gtk::Label,
    // swap_progress: gtk::ProgressBar,
    // swap_label: gtk::Label,
    // disk_progress: gtk::ProgressBar,
    // disk_label: gtk::Label,
}