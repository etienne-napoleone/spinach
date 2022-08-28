use std::thread::sleep;
use std::time::Duration;

use spinach::Spinach;

fn main() {
    let spinner = Spinach::new().text("hey").start();
    wait();
    spinner.text("hi").color("red").update();
    wait();
    spinner.stop();
}

fn wait() {
    sleep(Duration::from_secs(2));
}
