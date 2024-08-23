use std::thread::sleep;
use std::time::Duration;

use spinach::Color::Green;
use spinach::Spinner;

fn main() {
    let spinner = Spinner::new("Cutting spinaches")
        .color(Green)
        .frames_duration(30)
        .start();

    sleep(Duration::from_secs(1));
    spinner.text("Cutting tomatoes").update();

    sleep(Duration::from_secs(1));
    spinner.text("Vegetables cut").symbol("🔪").stop();

    let spinner = Spinner::new("Cooking vegetables")
        .color(Green)
        .frames_duration(30)
        .start();

    sleep(Duration::from_secs(1));
    spinner.text("Vegetables cooked").symbol("🍲").stop();

    sleep(Duration::from_secs(1));
}
