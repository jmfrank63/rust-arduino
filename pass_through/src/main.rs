#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut adc = arduino_hal::adc::Adc::new(dp.ADC, Default::default());
    let a0 = pins.a0.into_analog_input(&mut adc);

    // Convert pins 2 to 11 to output pins
    let mut output_pins = (
        pins.d2.into_output(),
        pins.d3.into_output(),
        pins.d4.into_output(),
        pins.d5.into_output(),
        pins.d6.into_output(),
        pins.d7.into_output(),
        pins.d8.into_output(),
        pins.d9.into_output(),
        pins.d10.into_output(),
        pins.d11.into_output(),
    );

    let mut control = pins.d12.into_output();

    loop {
        // Read from the ADC and map the value to a range of 0 to 1023.
        let adc_value = adc.read_blocking(&a0);
        control.set_low();
        // Output the ADC value to pins 2 to 11
        for i in 0..10 {
            match i {
                0 => {
                    if adc_value & (1 << 0) != 0 {
                        output_pins.0.set_high();
                    } else {
                        output_pins.0.set_low();
                    }
                }
                1 => {
                    if adc_value & (1 << 1) != 0 {
                        output_pins.1.set_high();
                    } else {
                        output_pins.1.set_low();
                    }
                }
                2 => {
                    if adc_value & (1 << 2) != 0 {
                        output_pins.2.set_high();
                    } else {
                        output_pins.2.set_low();
                    }
                }
                3 => {
                    if adc_value & (1 << 3) != 0 {
                        output_pins.3.set_high();
                    } else {
                        output_pins.3.set_low();
                    }
                }
                4 => {
                    if adc_value & (1 << 4) != 0 {
                        output_pins.4.set_high();
                    } else {
                        output_pins.4.set_low();
                    }
                }
                5 => {
                    if adc_value & (1 << 5) != 0 {
                        output_pins.5.set_high();
                    } else {
                        output_pins.5.set_low();
                    }
                }
                6 => {
                    if adc_value & (1 << 6) != 0 {
                        output_pins.6.set_high();
                    } else {
                        output_pins.6.set_low();
                    }
                }
                7 => {
                    if adc_value & (1 << 7) != 0 {
                        output_pins.7.set_high();
                    } else {
                        output_pins.7.set_low();
                    }
                }
                8 => {
                    if adc_value & (1 << 8) != 0 {
                        output_pins.8.set_high();
                    } else {
                        output_pins.8.set_low();
                    }
                }
                9 => {
                    if adc_value & (1 << 9) != 0 {
                        output_pins.9.set_high();
                    } else {
                        output_pins.9.set_low();
                    }
                }
                _ => {}
            }
        }
        control.set_high();
        arduino_hal::delay_ms(10);
    }

}
