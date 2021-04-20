use crate::send_byte;
use crate::{
    // send_one,
    send_reset,
    // send_zero,
};
use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portd::PD6;
use arduino_uno::prelude::*;

pub const LED_STRIP_LENGTH: usize = 64;
pub type LedStripBuffer = [Color; LED_STRIP_LENGTH];

#[derive(Copy, Clone)]
pub struct Color {
    pub green: u8,
    pub blue: u8,
    pub red: u8,
    pub white: u8,
}

impl Color {
    pub fn write(&self, led_pin: &mut PD6<Output>) {
        send_byte(self.green, led_pin);
        send_byte(self.red, led_pin);
        send_byte(self.blue, led_pin);
        send_byte(self.white, led_pin);
    }

    // pub fn black() -> Self {
    //     Self {
    //         green: 0,
    //         blue: 0,
    //         red: 0,
    //         white: 0,
    //     }
    // }

    pub fn red() -> Self {
        Self {
            green: 0,
            blue: 0,
            red: 1,
            white: 0,
        }
    }

    // pub fn shift(&mut self, by: u8) {
    //     self.red = self.red.wrapping_sub(by);
    // }
}

pub fn write(strip: &LedStripBuffer, led_pin: &mut PD6<Output>) {
    for (_idx, led) in strip.iter().enumerate() {
        led.write(led_pin);
        // if idx % 3 == 0 && idx != 0 {
        //     send_reset!(led_pin);
        // }
    }
    send_reset!(led_pin);
}
