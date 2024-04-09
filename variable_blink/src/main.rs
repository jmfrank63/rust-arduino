#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    let mut adc = arduino_hal::adc::Adc::new(dp.ADC, Default::default());
    let a0 = pins.a0.into_analog_input(&mut adc);
    loop {
        // Read from the ADC and map the value to a range of 0 to 1023.
        let delay = adc.read_blocking(&a0);

        led.toggle();
        arduino_hal::delay_ms(delay);
    }
}
