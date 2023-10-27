mod fileSaver;
mod keyMonitor; // Import the Keymonitor module from keyMonitor.rs

fn main() {
    let mut keymonitor: keyMonitor::Keymonitor = keyMonitor::Keymonitor::new();
    keymonitor.main_loop()
}
