use std::thread::sleep;
use std::time::Duration;

use spinach::Spinner;

fn main() {
    let spinner = Spinner::new().text("lol").start();
    pause();
    spinner.text("kek").update();
    pause();
    spinner.stop();
}

fn pause() {
    sleep(Duration::from_secs(2));
}
