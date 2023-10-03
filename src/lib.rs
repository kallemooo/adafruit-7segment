// Copyright (c) 2020 Karl Thorén <karl.h.thoren@gmail.com>
// Copyright (c) 2019 cs2dsb
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # adafruit-7segment backpack Hal
//!
//! Additional features on top of the [`ht16k33` crate](https://crates.io/crates/ht16k33) to drive an [Adafruit 7-segment LED Alphanumeric Backpack](https://learn.adafruit.com/adafruit-led-backpack/0-dot-56-seven-segment-backpack) using traits from `embedded-hal`.
//! Derived from the [`adafruit-alphanum4` crate](https://crates.io/crates/adafruit-alphanum4) and modified for the 7-segment backpacks.
//!
//! ## Features
//! * Sending a `u8` to one of the 4 segments. Limited to 0x00 to 0x0F.
//! * Sending an `AsciiChar` to one of the 4 segments. Limited to ascii hex chars and - sign.
//! * Setting or unsetting the dot associated with one of the 4 segments.
//! * Setting or unsetting the colon.
//! * Formatting a `f32` to 1 to 4 segments
//!
//! # Usage
//!
//! ## Embedded platforms
//! ### Example on a STM32F4-Discovery board
//! For examples on other platforms see the [`ht16k33` crate](https://crates.io/crates/ht16k33).
//!
//! `Cargo.toml` dependencies example:
//! ```toml
//! [dependencies]
//! htk16k33 = { version = "0.4.0", default-features = false }
//! adafruit-7segment = { version = "0.1", default-features = false  }
//! embedded-hal = "0.2.3"
//! cortex-m = "0.6.2"
//! cortex-m-rt = "0.6.12"
//! panic-halt = "0.2.0"
//!
//! [dependencies.stm32f4xx-hal]
//! version = "0.8"
//! features = ["rt", "stm32f407"]
//!```
//! Test code:
//!```!ignore
//! #![no_main]
//! #![no_std]
//!
//! use panic_halt as _;
//!
//! use cortex_m;
//! use cortex_m_rt::entry;
//! use stm32f4xx_hal as hal;
//!
//! use crate::hal::{i2c::I2c, prelude::*, stm32};
//! use ht16k33::{HT16K33, Dimming, Display};
//! use adafruit_7segment::{SevenSegment, Index};
//! pub use ascii::{ToAsciiChar, AsciiChar};
//!
//! #[entry]
//! fn main() -> ! {
//!  if let (Some(dp), Some(cp)) = (
//!    stm32::Peripherals::take(),
//!    cortex_m::peripheral::Peripherals::take(),
//!  ) {
//!    // Set up the system clock. We want to run at 48MHz for this one.
//!    let rcc = dp.RCC.constrain();
//!    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();
//!
//!    const DISP_I2C_ADDR: u8 = 112;
//!
//!    // Set up I2C - SCL is PB8 and SDA is PB7; they are set to Alternate Function 4
//!    // as per the STM32F407 datasheet.
//!    let gpiob = dp.GPIOB.split();
//!    let scl = gpiob.pb8.into_alternate_af4().set_open_drain();
//!    let sda = gpiob.pb7.into_alternate_af4().set_open_drain();
//!    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks);
//!
//!    let mut ht16k33 = HT16K33::new(i2c, DISP_I2C_ADDR);
//!    ht16k33.initialize().expect("Failed to initialize ht16k33");
//!    ht16k33.set_display(Display::ON).expect("Could not turn on the display!");
//!    ht16k33.set_dimming(Dimming::BRIGHTNESS_MIN).expect("Could not set dimming!");
//!
//!    // Sending individual digits
//!    ht16k33.update_buffer_with_digit(Index::One, 1);
//!    ht16k33.update_buffer_with_digit(Index::Two, 2);
//!    ht16k33.update_buffer_with_digit(Index::Three, 3);
//!    ht16k33.update_buffer_with_digit(Index::Four, 4);
//!
//!    // Sending ascii
//!    ht16k33.update_buffer_with_char(Index::One, AsciiChar::new('A'));
//!    ht16k33.update_buffer_with_char(Index::Two, AsciiChar::new('B'));
//!
//!    // Setting the decimal point
//!    ht16k33.update_buffer_with_dot(Index::Two, true);
//!
//!    // Formatting a float using the whole display
//!    ht16k33.update_buffer_with_float(Index::One, -3.14, 2, 10).unwrap();
//!
//!    // Putting a character in front of a float
//!    ht16k33.update_buffer_with_char(Index::One, AsciiChar::new('b'));
//!    // Display will read "b-3.1"
//!    ht16k33.update_buffer_with_float(Index::Two, -3.14, 2, 10).unwrap();
//!
//!    // This will panic because there aren't enough digits to display this number
//!    ht16k33.update_buffer_with_float(Index::One, 12345., 0, 10).expect("Oops");
//!
//!    // Note: none of the above methods actually commit the buffer to the display,
//!    // call write_display_buffer to actually send it to the display
//!    ht16k33.write_display_buffer().unwrap()
//!   }
//! loop {}
//! }
//!```
//! ## All platforms, using I2C simulation
//!```
//! use ht16k33::i2c_mock::I2cMock;
//! use ht16k33::{HT16K33, Dimming, Display};
//! use adafruit_7segment::{SevenSegment, Index};
//!
//! // The I2C device address.
//! const DISP_I2C_ADDR: u8 = 112;
//!
//! // Create a mock I2C device.
//! let mut i2c = I2cMock::new();
//!
//! let mut ht16k33 = HT16K33::new(i2c, DISP_I2C_ADDR);
//! ht16k33.initialize().expect("Failed to initialize ht16k33");
//! ht16k33.set_display(Display::ON).expect("Could not turn on the display!");
//! ht16k33.set_dimming(Dimming::BRIGHTNESS_MIN).expect("Could not set dimming!");
//!
//! // Sending individual digits
//! ht16k33.update_buffer_with_digit(Index::One, 1);
//! ht16k33.update_buffer_with_digit(Index::Two, 2);
//! ht16k33.update_buffer_with_digit(Index::Three, 3);
//! ht16k33.update_buffer_with_digit(Index::Four, 4);
//!
//! // Note: none of the above methods actually commit the buffer to the display,
//! // call write_display_buffer to actually send it to the display
//! ht16k33.write_display_buffer().unwrap()
//!```
//! ## Performance warning
//!
//! Due to the api of the ht16k33 crate the display buffer is not directly accessible so each LED that makes up the character is updated sequentially. The way the hardware on this backpack is set up allows a character to be updated by setting a single 16-bit value in the buffer. Iterating over each bit of the 16 every update is clearly not optimal but it's sufficiently fast for my current usage. If the ht16k33 crate is updated to grant mut access to the buffer this can be improved.

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![doc(html_root_url = "https://docs.rs/adafruit-7segment/0.1.0")]
#![cfg_attr(not(feature = "std"), no_std)]

mod fonts;
use fonts::*;

pub use ascii::{AsciiChar, ToAsciiChar};
use ht16k33::{DisplayData, DisplayDataAddress, LedLocation, COMMONS_SIZE, HT16K33};

/// Possible errors returned by this crate.
#[derive(Debug)]
pub enum Error {
    /// Error indicating there aren't enough digits to display the given float value.
    InsufficientDigits,
    /// Error indicating that the input cannot be displayed.
    NotValidChar,
}

/// Trait enabling using the Adafruit 7-segment LED numeric Backpack.
pub trait SevenSegment {
    /// Update the buffer with a digit value (0 to F) at the specified index.
    fn update_buffer_with_digit(&mut self, index: Index, value: u8);
    /// Update the buffer to turn the . on or off at the specified index.
    fn update_buffer_with_dot(&mut self, index: Index, dot_on: bool);
    /// Update the buffer to turn the : on or off.
    fn update_buffer_with_colon(&mut self, colon_on: bool);
    /// Update the buffer with an ascii character at the specified index.
    fn update_buffer_with_char(&mut self, index: Index, value: AsciiChar) -> Result<(), Error>;
    /// Update the buffer with a formatted float not starting before the specified index.
    fn update_buffer_with_float(
        &mut self,
        index: Index,
        value: f32,
        fractional_digits: u8,
        base: u8,
    ) -> Result<(), Error>;
}

/// The index of a segment
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Index {
    /// First digit
    One,
    /// Second digit
    Two,
    /// Third digit
    Three,
    /// Fourth digit
    Four,
}

impl From<Index> for u8 {
    fn from(i: Index) -> u8 {
        match i {
            Index::One => 0,
            Index::Two => 1,
            Index::Three => 2,
            Index::Four => 3,
        }
    }
}

impl From<u8> for Index {
    fn from(v: u8) -> Index {
        match v {
            0 => Index::One,
            1 => Index::Two,
            2 => Index::Three,
            3 => Index::Four,
            _ => panic!("Invalid index > 3"),
        }
    }
}

const MINUS_SIGN: u8 = 0x40;

const DOT_BIT: u8 = 7;

const COLON_BIT: u8 = 1;

fn set_bit<I2C>(display: &mut HT16K33<I2C>, index: u8, bit: u8, on: bool) {
    debug_assert!((bit as usize) < (COMMONS_SIZE * 2));
    let index = index * 2;
    let row = DisplayDataAddress::from_bits_truncate(if bit < 8 { index } else { index + 1 });
    let common = DisplayData::from_bits_truncate(1 << (bit % 8));
    display.update_display_buffer(LedLocation { row, common }, on);
}

fn update_bits<I2C>(display: &mut HT16K33<I2C>, index: Index, bits: u8) {
    let pos: u8;
    if index > Index::Two {
        // Move one step to compensate for colon at pos 2.
        pos = u8::from(index) + 1u8;
    } else {
        pos = index.into();
    }
    for i in 0..8 {
        let on = ((bits >> i) & 1) == 1;
        set_bit(display, pos, i, on);
    }
}

impl<I2C> SevenSegment for HT16K33<I2C> {
    /// Update the buffer with a hex digit value (0x00 to 0x0F) at the specified index
    /// # Arguments
    ///
    /// * `index` - Digit index.
    /// * `value` - Value 0x00 to 0x0F.
    ///
    /// # Examples
    ///
    /// ```
    /// use ht16k33::i2c_mock::I2cMock;
    /// use ht16k33::HT16K33;
    /// use adafruit_7segment::{SevenSegment, Index};
    ///
    /// // Create an I2C device.
    /// let mut i2c = I2cMock::new();
    ///
    /// // The I2C device address.
    /// const DISP_I2C_ADDR: u8 = 112;
    ///
    /// let mut ht16k33 = HT16K33::new(i2c, DISP_I2C_ADDR);
    ///
    /// // Set first digit to 9.
    /// ht16k33.update_buffer_with_digit(Index::One, 9);
    /// ```
    fn update_buffer_with_digit(&mut self, index: Index, value: u8) {
        let value = value as usize;
        assert!(value < HEX_NUMBER_FONT_TABLE.len());
        let bits = HEX_NUMBER_FONT_TABLE[value];
        update_bits(self, index, bits);
    }

    /// Update the buffer to turn the . on or off at the specified index
    /// # Arguments
    ///
    /// * `index` - Digit index.
    /// * `dot_on` - Enable or disable the dot.
    ///
    /// # Examples
    ///
    /// ```
    /// use ht16k33::i2c_mock::I2cMock;
    /// use ht16k33::HT16K33;
    /// use adafruit_7segment::{SevenSegment, Index};
    ///
    /// // Create an I2C device.
    /// let mut i2c = I2cMock::new();
    ///
    /// // The I2C device address.
    /// const DISP_I2C_ADDR: u8 = 112;
    ///
    /// let mut ht16k33 = HT16K33::new(i2c, DISP_I2C_ADDR);
    ///
    /// // Enable dot for first digit.
    /// ht16k33.update_buffer_with_dot(Index::One, true);
    /// ```
    fn update_buffer_with_dot(&mut self, index: Index, dot_on: bool) {
        let pos: u8;
        if index > Index::Two {
            // Move one step to compensate for colon at pos 2.
            pos = u8::from(index) + 1u8;
        } else {
            pos = index.into();
        }
        set_bit(self, pos, DOT_BIT, dot_on);
    }

    /// Update the buffer to turn the : on or off.
    /// # Arguments
    ///
    /// * `colon_on` - Enable or disable the colon.
    ///
    /// # Examples
    ///
    /// ```
    /// use ht16k33::i2c_mock::I2cMock;
    /// use ht16k33::HT16K33;
    /// use adafruit_7segment::{SevenSegment, Index};
    ///
    /// // Create an I2C device.
    /// let mut i2c = I2cMock::new();
    ///
    /// // The I2C device address.
    /// const DISP_I2C_ADDR: u8 = 112;
    ///
    /// let mut ht16k33 = HT16K33::new(i2c, DISP_I2C_ADDR);
    ///
    /// // Enable the colon.
    /// ht16k33.update_buffer_with_colon(true);
    /// ```
    fn update_buffer_with_colon(&mut self, colon_on: bool) {
        // The colon is at address 2.
        set_bit(self, 2u8, COLON_BIT, colon_on);
    }

    /// Update the buffer with an ascii character at the specified index.
    /// # Arguments
    ///
    /// * `index` - Digit index.
    /// * `value` - Ascii character.
    ///
    /// # Examples
    ///
    /// ```
    /// use ht16k33::i2c_mock::I2cMock;
    /// use ht16k33::HT16K33;
    /// use adafruit_7segment::{SevenSegment, Index, AsciiChar};
    ///
    /// // Create an I2C device.
    /// let mut i2c = I2cMock::new();
    ///
    /// // The I2C device address.
    /// const DISP_I2C_ADDR: u8 = 112;
    ///
    /// let mut ht16k33 = HT16K33::new(i2c, DISP_I2C_ADDR);
    ///
    /// // Set first digit to 'c'.
    /// ht16k33.update_buffer_with_char(Index::One, AsciiChar::new('c')).expect("Failed to encode char to buffer!");
    /// ```
    fn update_buffer_with_char(&mut self, index: Index, value: AsciiChar) -> Result<(), Error> {
        if value.is_ascii_hexdigit() {
            let val: u8;
            if value.is_ascii_digit() {
                // 0-9 converted to hex value
                val = value.as_byte() - b'0';
            } else {
                // a-f or A-F converted to hex value
                val = 0x0A + (value.to_ascii_uppercase().as_byte() - b'A');
            }
            let val = val as usize;
            assert!(val < HEX_NUMBER_FONT_TABLE.len());
            let bits = HEX_NUMBER_FONT_TABLE[val];
            update_bits(self, index, bits);
        } else if value == '-' {
            update_bits(self, index, MINUS_SIGN);
        } else {
            return Err(Error::NotValidChar);
        }

        Ok(())
    }

    /// Update the buffer with a formatted float not starting before the specified index
    /// The logic for this is copied mostly from from the adafruit library. Only difference is this allows the start index to be > 0
    ///
    /// # Arguments
    ///
    /// * `index` - Digit index.
    /// * `value` - float value.
    /// * `fractional_digits` - Number of fractional digits.
    /// * `base` - Base to use.
    ///
    /// # Examples
    ///
    /// ```
    /// use ht16k33::i2c_mock::I2cMock;
    /// use ht16k33::HT16K33;
    /// use adafruit_7segment::{SevenSegment, Index};
    ///
    /// // Create an I2C device.
    /// let mut i2c = I2cMock::new();
    ///
    /// // The I2C device address.
    /// const DISP_I2C_ADDR: u8 = 112;
    ///
    /// let mut ht16k33 = HT16K33::new(i2c, DISP_I2C_ADDR);
    ///
    /// // Write 9.9 from pos 2
    /// ht16k33.update_buffer_with_float(Index::Two, 9.9, 1, 10);
    /// ```
    fn update_buffer_with_float(
        &mut self,
        index: Index,
        mut value: f32,
        mut fractional_digits: u8,
        base: u8,
    ) -> Result<(), Error> {
        let index = u8::from(index);

        // Available digits on display
        let mut numeric_digits = 4 - index;

        let is_negative = if value < 0. {
            // The sign will take up one digit
            numeric_digits -= 1;
            // Flip the sign to do the rest of the formatting
            value *= -1.;
            true
        } else {
            false
        };

        let base = base as u32;
        let basef = base as f32;

        // Work out the multiplier needed to get all fraction digits into an integer
        let mut to_int_factor = base.pow(fractional_digits as u32) as f32;

        // Get an integer containing digits to be displayed
        let mut display_number = ((value * to_int_factor) + 0.5) as u32;

        // Calculate the upper bound given the number of digits available
        let too_big = base.pow(numeric_digits as u32);

        // If the number is too large, reduce fractional digits
        while display_number >= too_big {
            fractional_digits -= 1;
            to_int_factor /= basef;
            display_number = ((value * to_int_factor) + 0.5) as u32;
        }

        // Did we lose the decimal?
        if to_int_factor < 1. {
            return Err(Error::InsufficientDigits);
        }

        // Digit we're working on, less the start position
        let mut display_pos = (3 - index) as i8;

        if display_number == 0 {
            // Write out the 0
            self.update_buffer_with_digit((index + (display_pos as u8)).into(), 0);
            // Move the current pos along
            display_pos -= 1;
        } else {
            let mut i = 0;
            while display_number != 0 || i <= fractional_digits {
                let digit_index: Index = (index + (display_pos as u8)).into();
                // Write out the current digit
                self.update_buffer_with_digit(digit_index, (display_number % base) as u8);
                // Add the decimal if necessary
                if fractional_digits != 0 && i == fractional_digits {
                    self.update_buffer_with_dot(digit_index, true);
                }
                // Move the current pos along
                display_pos -= 1;
                // Move the number along
                display_number /= base;
                i += 1;
            }
        }

        if is_negative {
            // Add the minus sign
            update_bits(self, (index + (display_pos as u8)).into(), MINUS_SIGN);
            // Move the current pos along
            display_pos -= 1;
        }

        // Clear any remaining segments
        while display_pos >= 0 {
            update_bits(self, (index + (display_pos as u8)).into(), 0);
            // Move the current pos along
            display_pos -= 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use embedded_hal_mock as hal;

    use self::hal::i2c::Mock as I2cMock;
    use super::*;

    const ADDRESS: u8 = 0;

    #[test]
    fn update_buffer_with_dot() {
        let expectations = [];

        let mut i2c = I2cMock::new(&expectations);
        let mut ht16k33 = HT16K33::new(i2c, ADDRESS);

        ht16k33.update_buffer_with_dot(Index::One, true);
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b1000_0000);
        assert_eq!(ht16k33.display_buffer()[1].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[2].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[3].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[4].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[5].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[6].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[7].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[8].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[9].bits(), 0b0000_0000);

        ht16k33.update_buffer_with_dot(Index::Two, true);
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b1000_0000);
        assert_eq!(ht16k33.display_buffer()[1].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[2].bits(), 0b1000_0000);
        assert_eq!(ht16k33.display_buffer()[3].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[4].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[5].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[6].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[7].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[8].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[9].bits(), 0b0000_0000);

        ht16k33.update_buffer_with_dot(Index::Three, true);
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b1000_0000);
        assert_eq!(ht16k33.display_buffer()[1].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[2].bits(), 0b1000_0000);
        assert_eq!(ht16k33.display_buffer()[3].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[4].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[5].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[6].bits(), 0b1000_0000);
        assert_eq!(ht16k33.display_buffer()[7].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[8].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[9].bits(), 0b0000_0000);

        ht16k33.update_buffer_with_dot(Index::Four, true);
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b1000_0000);
        assert_eq!(ht16k33.display_buffer()[1].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[2].bits(), 0b1000_0000);
        assert_eq!(ht16k33.display_buffer()[3].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[4].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[5].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[6].bits(), 0b1000_0000);
        assert_eq!(ht16k33.display_buffer()[7].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[8].bits(), 0b1000_0000);
        assert_eq!(ht16k33.display_buffer()[9].bits(), 0b0000_0000);

        i2c = ht16k33.destroy();
        i2c.done();
    }

    #[test]
    fn update_buffer_with_colon() {
        let expectations = [];

        let mut i2c = I2cMock::new(&expectations);
        let mut ht16k33 = HT16K33::new(i2c, ADDRESS);

        // Enable colon
        ht16k33.update_buffer_with_colon(true);
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[1].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[2].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[3].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[4].bits(), 0b0000_0010);
        assert_eq!(ht16k33.display_buffer()[5].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[6].bits(), 0b0000_0000);

        i2c = ht16k33.destroy();
        i2c.done();
    }

    #[test]
    fn update_buffer_with_digit() {
        let expectations = [];

        let mut i2c = I2cMock::new(&expectations);
        let mut ht16k33 = HT16K33::new(i2c, ADDRESS);

        // Write an A
        ht16k33.update_buffer_with_digit(Index::One, 0x0A);
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b0111_0111);

        // Write an B
        ht16k33.update_buffer_with_digit(Index::One, 0x0B);
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b0111_1100);

        // Write an 0
        ht16k33.update_buffer_with_digit(Index::One, 0x00);
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b0011_1111);

        // Write an 9
        ht16k33.update_buffer_with_digit(Index::One, 0x09);
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b0110_1111);

        i2c = ht16k33.destroy();
        i2c.done();
    }

    #[test]
    fn update_buffer_with_char() {
        let expectations = [];

        let mut i2c = I2cMock::new(&expectations);
        let mut ht16k33 = HT16K33::new(i2c, ADDRESS);

        // Write an A
        assert!(ht16k33
            .update_buffer_with_char(Index::One, AsciiChar::new('A'))
            .is_ok());
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b0111_0111);

        // Write an a
        assert!(ht16k33
            .update_buffer_with_char(Index::One, AsciiChar::new('a'))
            .is_ok());
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b0111_0111);

        // Write an B
        assert!(ht16k33
            .update_buffer_with_char(Index::One, AsciiChar::new('B'))
            .is_ok());
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b0111_1100);

        // Write an b
        assert!(ht16k33
            .update_buffer_with_char(Index::One, AsciiChar::new('b'))
            .is_ok());
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b0111_1100);

        // Write an 0
        assert!(ht16k33
            .update_buffer_with_char(Index::One, AsciiChar::new('0'))
            .is_ok());
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b0011_1111);

        // Write an 9
        assert!(ht16k33
            .update_buffer_with_char(Index::One, AsciiChar::new('9'))
            .is_ok());
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b0110_1111);

        // Write an -
        assert!(ht16k33
            .update_buffer_with_char(Index::One, AsciiChar::new('-'))
            .is_ok());
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b0100_0000);

        i2c = ht16k33.destroy();
        i2c.done();
    }

    #[test]
    fn update_buffer_with_float() {
        let expectations = [];

        let mut i2c = I2cMock::new(&expectations);
        let mut ht16k33 = HT16K33::new(i2c, ADDRESS);

        assert!(ht16k33
            .update_buffer_with_float(Index::One, 99.9, 2, 10)
            .is_ok());
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b0110_1111);
        assert_eq!(ht16k33.display_buffer()[1].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[2].bits(), 0b1110_1111);
        assert_eq!(ht16k33.display_buffer()[3].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[4].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[5].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[6].bits(), 0b0110_1111);
        assert_eq!(ht16k33.display_buffer()[7].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[8].bits(), 0b0011_1111);
        assert_eq!(ht16k33.display_buffer()[9].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[10].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[11].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[12].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[13].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[14].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[15].bits(), 0b0000_0000);

        assert!(ht16k33
            .update_buffer_with_float(Index::One, -99.9, 2, 10)
            .is_ok());
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b0100_0000);
        assert_eq!(ht16k33.display_buffer()[1].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[2].bits(), 0b0110_1111);
        assert_eq!(ht16k33.display_buffer()[3].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[4].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[5].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[6].bits(), 0b1110_1111);
        assert_eq!(ht16k33.display_buffer()[7].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[8].bits(), 0b0110_1111);
        assert_eq!(ht16k33.display_buffer()[9].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[10].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[11].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[12].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[13].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[14].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[15].bits(), 0b0000_0000);

        ht16k33.clear_display_buffer();
        assert!(ht16k33
            .update_buffer_with_float(Index::Two, 9.9, 1, 10)
            .is_ok());
        assert_eq!(ht16k33.display_buffer()[0].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[1].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[2].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[3].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[4].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[5].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[6].bits(), 0b1110_1111);
        assert_eq!(ht16k33.display_buffer()[7].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[8].bits(), 0b0110_1111);
        assert_eq!(ht16k33.display_buffer()[9].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[10].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[11].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[12].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[13].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[14].bits(), 0b0000_0000);
        assert_eq!(ht16k33.display_buffer()[15].bits(), 0b0000_0000);

        i2c = ht16k33.destroy();
        i2c.done();
    }
}
