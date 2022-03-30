use std::thread::sleep;
use std::time::Duration;

use spinach::Spinach;

fn main() {
    for message in vec![
        "Get a cutting board",
        "Chop spinach",
        "Prepare pan",
        "Sauté spinach",
    ] {
        let s = Spinach::new(message);
        sleep(Duration::from_secs(2));
        s.succeed(None);
    }
}
