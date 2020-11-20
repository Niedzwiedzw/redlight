#![no_std]
#![no_main]

use panic_halt as _;
use arduino_uno::spi::{Spi, Settings};
use arduino_uno::hal::delay::Delay;
use ws2812_spi::prerendered::{Ws2812, Timing};
use smart_leds::{SmartLedsWrite, RGB8};
use arduino_uno::prelude::*;

const MAX: usize = 16;
const COLOR1: RGB8 = RGB8 {
    r: 0x00,
    g: 0xc3 / 5,
    b: 0x36 / 5,
};
const COLOR2: RGB8 = RGB8 {
    r: 0x15,
    g: 0x00,
    b: 0x00,
};
#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let cp = arduino_uno::Peripherals::take().unwrap();
    // Configure pins for SPI

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    pins.d10.into_output(&mut pins.ddr); // CS must be set to output mode.
    // Digital pin 13 is also connected to an onboard LED marked "L"

    let mut settings = Settings::default();
    settings.clock = arduino_uno::spi::SerialClockRate::OscfOver4;
    let mut spi = Spi::new(
        dp.SPI,
        pins.d13.into_output(&mut pins.ddr),
        pins.d11.into_output(&mut pins.ddr),
        pins.d12.into_pull_up_input(&mut pins.ddr),
        settings,
    );
    let mut delay: Delay<arduino_uno::hal::clock::MHz16> = Delay::new();
    let mut data: [RGB8; MAX] = [(0, 0, 0).into(); MAX];
    let mut main = 0;
    let mut data_buff = [0u8; MAX * 3];
    let mut ws = Ws2812::new(spi, Timing::new(4u32).unwrap(), &mut data_buff);
    let mut up = true;
    loop {
            for i in 0..MAX {
                let distance = (main as i32 - i as i32).abs() as u8;
                let c1 = (
                    COLOR1.r as u32 * (MAX as u32 - distance as u32) / MAX as u32,
                    COLOR1.g as u32 * (MAX as u32 - distance as u32) / MAX as u32,
                    COLOR1.b as u32 * (MAX as u32 - distance as u32) / MAX as u32,
                );
                let c2 = (
                    COLOR2.r as u32 * distance as u32 / MAX as u32,
                    COLOR2.g as u32 * distance as u32 / MAX as u32,
                    COLOR2.b as u32 * distance as u32 / MAX as u32,
                );
                let ct = (
                    (c1.0 + c2.0) as u8,
                    (c1.1 + c2.1) as u8,
                    (c1.2 + c2.2) as u8,
                )
                    .into();
                data[i] = ct;
            }
            if up {
                if main == MAX - 1 {
                    up = false;
                    main -= 2;
                }
                main += 1;
            } else {
                if main == 0 {
                    up = true;
                    main += 2;
                }
                main -= 1;
            }
        ws.write(data.iter().cloned()).unwrap();
        delay.delay_ms(100u8);
    }
}
