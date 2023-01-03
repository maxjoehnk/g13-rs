# g13-rs

libusb based crate to communicate with a Logitech G13 without accompanying drivers.

## Installation

`cargo add g13`

## Usage

To access a G13 device instantiate a new `G13Manager` and call `discover` on it.

```rust
use g13::*;

let mut manager = G13Manager::new()?;
let mut devices: Vec<G13> = manager.discover()?;
```

### LCD

Use `clear_lcd` to clear the lcd.

You can use `write_lcd` with a 960 bytes large buffer to write to the display.

### Keyboard

To set the keyboard color use `set_key_color` with a tuple of RGB bytes.

The Mode LEDs (M1, M2, M3 and MR) can be set with the ModeLeds bit flags.

To read keyboard input call `read` which will wait for the next interrupt until the given `timeout` is reached.
