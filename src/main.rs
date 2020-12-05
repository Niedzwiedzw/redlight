#![no_std]
#![no_main]
mod color;
mod ledstrip;
mod macros;
use arduino_uno::delay_ms;
use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portd::PD6;
use arduino_uno::prelude::*;
use panic_halt as _;



fn send_bit(bit: u8, led_pin: &mut PD6<Output>) {
    match bit {
        0 => {send_zero!(led_pin);},
        1 => {send_one!(led_pin);},
        _ => panic!()
    }
}

pub fn send_byte(byte: u8, led_pin: &mut PD6<Output>) {
    for x in (0..8).rev() {
        send_bit((byte >> x) & 0b00000001, led_pin);
    }
}

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();
    let mut pins = arduino_uno::Pins::new(peripherals.PORTB, peripherals.PORTC, peripherals.PORTD);
    let mut led_pin = pins.d6.into_output(&mut pins.ddr);

    let mut led_strip: ledstrip::LedStripBuffer = [ledstrip::Color::red(); ledstrip::LED_STRIP_LENGTH];
    ledstrip::write(&led_strip, &mut led_pin);
    
    loop {
        for _led in led_strip.iter_mut() {
            // led.shift(127);
        }
        delay_ms(20);
        ledstrip::write(&led_strip, &mut led_pin);
    }
}
