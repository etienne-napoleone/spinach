# 🥬 spinach

[![Crates.io](https://img.shields.io/crates/v/spinach)](https://crates.io/crates/spinach)
[![Docs.rs](https://img.shields.io/docsrs/spinach)](https://docs.rs/spinach)
[![License](https://img.shields.io/crates/l/spinach)](LICENSE)
[![CI](https://github.com/etienne-napoleone/spinach/actions/workflows/ci.yml/badge.svg)](https://github.com/etienne-napoleone/spinach/actions/workflows/ci.yml)

> Practical spinner for Rust — `v3` now with method chaining

<p align="center">
	<img src="https://raw.githubusercontent.com/etienne-napoleone/spinach/main/assets/screenshot.gif" width="550px" height="399px">
</p>

## Install

Add as a dependency to your `Cargo.toml`.

```toml
[dependencies]
spinach = "3"
```

## Usage

Basic example.

```rust
use spinach::Spinner;

fn main() {
	// Cut spinaches
    let s = Spinner::new("Cutting spinaches...").start();
	// Cut tomatoes
    s.text("Cutting tomatoes...").update();
	// We're done!
	s.text("Vegetables cut").symbol("🔪").stop();
}
```

### Starting

```rust
use spinach::{Color, Spinner};

// With custom text
let s = Spinner::new("workin'...").start();

// With custom text, spinner, spinner speed and spinner color
let symbols = vec!["▮","▯"];
let s = Spinner::new("blip... blop...")
    .color(Color::Red)
    .symbols(symbols)
    .frames_duration(80)
    .start();
```

### Updating

```rust
use spinach::{Color, Spinner};

let s = Spinner::new("workin'...").start();

// Updating text
s.text("new text").update();

// Updating color
s.color(Color::White).update();

// Updating spinner symbols
s.symbols(vec!["◐", "◓", "◑", "◒"]).update();

// Updating spinner speed
s.frames_duration(80).update();

// Updating multiple at once
s.text("new text").color(Color::Red);
```

### Stopping

```rust
use spinach::{Color, Spinner};

let s = Spinner::new("workin'...").start();

// Stop with final `✔` frame and green color.
s.text("gg!").success();

// Stop with final `✖` frame and red color.
s.text("ups").failure();

// Stop with final `⚠` frame and yellow color.
s.text("something may have happened?").warn();

// Stop with final `ℹ` frame and blue color.
s.text("notice").stop();

// Stop current spinner (sends update at the same time)
s.stop(); // freeze
s.text("spinach'd").symbol("🥬").stop(); // stop with the text "spinach'd" and a vegetable as the spinner
```

## FAQ

### How to avoid leaving terminal without prompt on interupt (ctrl^c)?

You can use a library like [`ctrlc`](https://crates.io/crates/ctrlc) to handle interupts.

The most basic way to handle it would be in conjuction with this lib QoL `show_cursor` function like this:

```rust
use spinach::{show_cursor, Spinner};

fn main() {
    ctrlc::set_handler(|| {
        show_cursor();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let s = Spinner::new("workin'...").start();
    // ...
```

## Related

Inspired by:

- [ora](https://github.com/sindresorhus/ora)
- [spinners](https://github.com/FGRibreau/spinners)
