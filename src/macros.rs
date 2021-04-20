#[macro_export]
macro_rules! send_zero {
    ($led_pin:ident) => {
        $led_pin.set_high().unwrap();
        arduino_uno::delay_us(1);
        $led_pin.set_low().unwrap();
        arduino_uno::delay_us(1);
    };
}
#[macro_export]
macro_rules! send_one {
    ($led_pin:ident) => {
        $led_pin.set_low().unwrap();
        arduino_uno::delay_us(1);
        $led_pin.set_high().unwrap();
        arduino_uno::delay_us(1);
    };
}
#[macro_export]
macro_rules! send_reset {
    ($led_pin:ident) => {
        $led_pin.set_low().unwrap();
        arduino_uno::delay_us(90);
    };
}
