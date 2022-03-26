# 🥬 spinach

[![Crates.io](https://img.shields.io/crates/v/spinach)](https://crates.io/crates/spinach)
[![Docs.rs](https://img.shields.io/docsrs/spinach)](https://docs.rs/spinach)
[![License](https://img.shields.io/crates/l/spinach/1.0.1)](LICENSE)
[![CI](https://github.com/etienne-napoleone/spinach/actions/workflows/ci.yml/badge.svg)](https://github.com/etienne-napoleone/spinach/actions/workflows/ci.yml)

> Practical spinner for Rust 

<p align="center">
	<img src="https://raw.githubusercontent.com/etienne-napoleone/spinach/main/assets/screenshot.png" width="550px" height="399px">
</p>

## Install

Add as a dependency to your `Cargo.toml`.

```toml
[dependencies]
spinach = "2"
```

## Usage

Basic example.

```rust
use std::thread::sleep;
use std::time::Duration;

use spinach::Spinach;

fn main() {
    let s = Spinach::new("Running task 1");
    sleep(Duration::from_secs(1));

    s.text("Running task 2");
    sleep(Duration::from_secs(1));

    s.succeed("Ran tasks successfully");
}
```

For general convenience, text can be passed as `String` or `&str`.
When an `Option`, can be passed as `String`, `&str` or `Option<String>`

### Creating

```rust
use spinach::{Color, Spinach, Spinner};

// Using defaults + custom text
let s = Spinach::new("custom text");

// Using custom spinner
let spinner = Spinner::new(vec!["▮","▯"], 80);
let s = Spinach::new_with(spinner, "custom text", Color::Red));

// Also with partial config (fallback to defaults)
let s = Spinach::new_with(None, "custom text", Color::Green);
```

### Updating

```rust
use spinach::{Color, Spinach};

let s = Spinach::new("custom text");

// Updating text
s.text("new text");

// Updating color
s.color(Color::White);

// Updating multiple
s.update_with("new text", Color::Red);

// Also with partial update (keep current)
s.update_with(None, Color::Red);
```

### Stopping

```rust
use spinach::{Color, Spinach};

let s = Spinach::new("custom text");

// Stop with final `✔` frame, green color and optional text change.
s.success("gg");

// Stop with final `✖` frame, red color and optional text change.
s.fail("ups");

// Stop with final `⚠` frame, yellow color and optional text change.
s.warn(None);

// Stop with final `ℹ` frame, blue color and optional text change.
s.info("notice");

// Stop current spinner (freeze the frame)
s.stop();

// Stopping with custom final frame, text and color
s.stop_with("🥬", "spinach'd", Color::Ignore);

// Also with partial update (keep current)
s.stop_with(None, None, Color::Blue);
```

### Freezing

Will freeze the current line with passed options and continue on a new line.

```rust
use spinach::{Color, Spinach};

let s = Spinach::new("Cutting spinaches");

// Similar to `stop_with`, but with an extra argument to change the spinner text.
s.freeze("🥬", "Spinaches cut", None, "Cutting carottes");
```

## FAQ

### How to avoid leaving terminal without prompt on interupt (ctrl^c)?

You can use a library like [`ctrlc`](https://crates.io/crates/ctrlc) to handle interupts.

The most basic way to handle it would be in conjuction with this lib QoL `show_cursor` function like this:

```rust
use spinach::{term, Spinach};

fn main() {
    ctrlc::set_handler(|| {
        term::show_cursor();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let s = Spinach::new("spinnin'");
    // ...
```

## Related

Inspired by:

- [ora](https://github.com/sindresorhus/ora)
- [spinners](https://github.com/FGRibreau/spinners)
