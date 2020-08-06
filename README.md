# adafruit-7segment backpack Hal

Additional features on top of the [`ht16k33` crate](https://crates.io/crates/ht16k33) to drive an [Adafruit 7-segment LED Alphanumeric Backpack](https://learn.adafruit.com/adafruit-led-backpack/0-dot-56-seven-segment-backpack) using traits from `embedded-hal`.
Derived from the [`adafruit-alphanum4` crate](https://crates.io/crates/adafruit-alphanum4) and modified for the 7-segment backpacks.

## Features
* Sending a `u8` to one of the 4 segments. Limited to 0x00 to 0x0F.
* Sending an `AsciiChar` to one of the 4 segments. Limited to ascii hex chars and - sign.
* Setting or unsetting the dot associated with one of the 4 segments.
* Setting or unsetting the colon.
* Formatting a `f32` to 1 to 4 segments

## Embedded platforms
### Example on a STM32F4-Discovery board
For examples on other platforms see the [`ht16k33` crate](https://crates.io/crates/ht16k33).

`Cargo.toml` dependencies example:
```toml
[dependencies]
htk16k33 = { version = "*", default-features = false }
adafruit-7segment = { version = "*", default-features = false  }
embedded-hal = "0.2.3"
cortex-m = "0.6.2"
cortex-m-rt = "0.6.12"
panic-halt = "0.2.0"

[dependencies.stm32f4xx-hal]
version = "0.8"
features = ["rt", "stm32f407"]
```
Test code:
```rust
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{i2c::I2c, prelude::*, stm32};
use ht16k33::{HT16K33, Dimming, Display};
use adafruit_7segment::{SevenSegment, Index};
pub use ascii::{ToAsciiChar, AsciiChar};

#[entry]
fn main() -> ! {
 if let (Some(dp), Some(cp)) = (
   stm32::Peripherals::take(),
   cortex_m::peripheral::Peripherals::take(),
 ) {
   // Set up the system clock. We want to run at 48MHz for this one.
   let rcc = dp.RCC.constrain();
   let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

   const DISP_I2C_ADDR: u8 = 112;

   // Set up I2C - SCL is PB8 and SDA is PB7; they are set to Alternate Function 4
   // as per the STM32F407 datasheet.
   let gpiob = dp.GPIOB.split();
   let scl = gpiob.pb8.into_alternate_af4().set_open_drain();
   let sda = gpiob.pb7.into_alternate_af4().set_open_drain();
   let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks);

   let mut ht16k33 = HT16K33::new(i2c, DISP_I2C_ADDR);
   ht16k33.initialize().expect("Failed to initialize ht16k33");
   ht16k33.set_display(Display::ON).expect("Could not turn on the display!");
   ht16k33.set_dimming(Dimming::BRIGHTNESS_MIN).expect("Could not set dimming!");

   // Sending individual digits
   ht16k33.update_buffer_with_digit(Index::One, 1);
   ht16k33.update_buffer_with_digit(Index::Two, 2);
   ht16k33.update_buffer_with_digit(Index::Three, 3);
   ht16k33.update_buffer_with_digit(Index::Four, 4);

   // Sending ascii
   ht16k33.update_buffer_with_char(Index::One, AsciiChar::new('A'));
   ht16k33.update_buffer_with_char(Index::Two, AsciiChar::new('B'));

   // Setting the decimal point
   ht16k33.update_buffer_with_dot(Index::Two, true);

   // Formatting a float using the whole display
   ht16k33.update_buffer_with_float(Index::One, -3.14, 2, 10).unwrap();

   // Putting a character in front of a float
   ht16k33.update_buffer_with_char(Index::One, AsciiChar::new('b'));
   // Display will read "b-3.1"
   ht16k33.update_buffer_with_float(Index::Two, -3.14, 2, 10).unwrap();

   // This will panic because there aren't enough digits to display this number
   ht16k33.update_buffer_with_float(Index::One, 12345., 0, 10).expect("Oops");

   // Note: none of the above methods actually commit the buffer to the display,
   // call write_display_buffer to actually send it to the display
   ht16k33.write_display_buffer().unwrap()
  }
loop {}
}
```
## All platforms, using I2C simulation
```rust
use ht16k33::i2c_mock::I2cMock;
use ht16k33::{HT16K33, Dimming, Display};
use adafruit_7segment::{SevenSegment, Index};

// The I2C device address.
const DISP_I2C_ADDR: u8 = 112;

// Create a mock I2C device.
let mut i2c = I2cMock::new();

let mut ht16k33 = HT16K33::new(i2c, DISP_I2C_ADDR);
ht16k33.initialize().expect("Failed to initialize ht16k33");
ht16k33.set_display(Display::ON).expect("Could not turn on the display!");
ht16k33.set_dimming(Dimming::BRIGHTNESS_MIN).expect("Could not set dimming!");

// Sending individual digits
ht16k33.update_buffer_with_digit(Index::One, 1);
ht16k33.update_buffer_with_digit(Index::Two, 2);
ht16k33.update_buffer_with_digit(Index::Three, 3);
ht16k33.update_buffer_with_digit(Index::Four, 4);

// Note: none of the above methods actually commit the buffer to the display,
// call write_display_buffer to actually send it to the display
ht16k33.write_display_buffer().unwrap()
```
## Performance warning

Due to the api of the ht16k33 crate the display buffer is not directly accessible so each LED that makes up the character is updated sequentially. The way the hardware on this backpack is set up allows a character to be updated by setting a single 16-bit value in the buffer. Iterating over each bit of the 16 every update is clearly not optimal but it's sufficiently fast for my current usage. If the ht16k33 crate is updated to grant mut access to the buffer this can be improved.
