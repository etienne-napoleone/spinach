use std::thread::sleep;
use std::time::Duration;

use spinach::Color::Green;
use spinach::Spinner;

fn main() {
    let spinner = Spinner::new()
        .color(Green)
        .text("Cutting spinaches")
        .frames_duration(30)
        .start();
    sleep(Duration::from_secs(1));
    spinner.text("Cutting tomatoes").update();
    sleep(Duration::from_secs(1));
    spinner.text("Vegetables cut").symbols(vec!["🔪"]).stop();

    let spinner = Spinner::new()
        .color(Green)
        .text("Cooking vegetables")
        .frames_duration(30)
        .start();
    sleep(Duration::from_secs(1));
    spinner.text("Vegetables cooked").symbols(vec!["🍲"]).stop();

    sleep(Duration::from_secs(1));
}
