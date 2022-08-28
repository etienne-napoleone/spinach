use std::thread::sleep;
use std::time::Duration;

use spinach::Spinner;

fn main() {
    let spinner = Spinner::new().text("hey").start();
    wait();
    spinner.text("hi").color("red").update();
    wait();
    spinner.stop();
}

fn wait() {
    sleep(Duration::from_secs(2));
}
