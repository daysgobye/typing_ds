mod file_saver;
mod key_monitor; // Import the Keymonitor module from keyMonitor.rs
mod letter_manager;

fn main() {
    let mut keymonitor: key_monitor::Keymonitor = key_monitor::Keymonitor::new();
    keymonitor.main_loop()
}
