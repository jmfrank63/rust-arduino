#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use filters::{HighPassFilter, LowPassFilter};
use panic_halt as _;

static mut FLAG: bool = false;

#[avr_device::entry]
fn main() -> ! {
    let dp = avr_device::atmega328p::Peripherals::take().unwrap();
    // Select ADC channel 0
    dp.ADC.admux.write(|w| w.refs().avcc().mux().adc0());
    // Enable the ADC and start conversion
    dp.ADC
        .adcsra
        .write(|w| w.aden().set_bit().adsc().set_bit().adps().prescaler_128());

    // Set PD2 to PD7 as output
    dp.PORTD.ddrd.write(|w| {
        w.pd2()
            .set_bit()
            .pd3()
            .set_bit()
            .pd4()
            .set_bit()
            .pd5()
            .set_bit()
            .pd6()
            .set_bit()
            .pd7()
            .set_bit()
    });
    // Set PB0 to PB4 as output PB5 as input
    dp.PORTB.ddrb.write(|w| {
        w.pb0()
            .set_bit()
            .pb1()
            .set_bit()
            .pb2()
            .set_bit()
            .pb3()
            .set_bit()
            .pb4()
            .set_bit()
            .pb5()
            .clear_bit()
    });

    // Set PC1 to PC5 as input
    dp.PORTC.ddrc.write(|w| {
        w.pc1()
            .clear_bit()
            .pc2()
            .clear_bit()
            .pc3()
            .clear_bit()
            .pc4()
            .clear_bit()
            .pc5()
            .clear_bit()
            .pc5()
            .clear_bit()
    });

    // Enable internal pull-up resistors for PC1 to PC5
    dp.PORTC.portc.write(|w| {
        w.pc1()
            .set_bit()
            .pc2()
            .set_bit()
            .pc3()
            .set_bit()
            .pc4()
            .set_bit()
            .pc5()
            .set_bit()
    });
    unsafe { avr_device::interrupt::enable() };
    // Enable pin change interrupt PCIE1 for PC1 to PC5
    dp.EXINT.pcmsk1.write(|w| w.pcint().bits(0b0011_1110));
    dp.EXINT.pcicr.write(|w| w.pcie().bits(0b0000_0010));

    // Read the alpha value and create the filter
    let alpha = (dp.PORTC.pinc.read().bits() & 0b00111110) >> 1;

    let mut lowpass_filter = LowPassFilter::new(alpha as u16);
    let mut highpass_filter = HighPassFilter::new(alpha as u16);
    let mut input_value: u16;

    loop {
        // read the alpha change flag
        let flag = unsafe { FLAG };

        // Set alpha value if it has changed
        if flag {
            avr_device::interrupt::free(|_| unsafe {
                FLAG = false;
            });
            // Read the alpha value and update the filter
            let alpha = (dp.PORTC.pinc.read().bits() & 0b00111110) >> 1;
            if dp.PORTB.pinb.read().pb5().bit_is_set() {
                highpass_filter.set_alpha(alpha as u16);
                highpass_filter.reset();
            } else {
                lowpass_filter.set_alpha(alpha as u16);
                lowpass_filter.reset();
            }
        }

        // Wait for the conversion to complete
        while dp.ADC.adcsra.read().adsc().bit_is_set() {}

        // Read the result
        input_value = dp.ADC.adc.read().bits();
        // Start the next conversion
        dp.ADC
            .adcsra
            .write(|w| w.aden().set_bit().adsc().set_bit().adps().prescaler_2());

        // Apply the filter dependent on the the PB5 value
        if dp.PORTB.pinb.read().pb5().bit_is_set() {
            highpass_filter.high_pass(&mut input_value);
        } else {
            lowpass_filter.low_pass(&mut input_value);
        }
        // Disable the latch takeover
        dp.PORTB.portb.modify(|_, w| w.pb4().clear_bit());
        // Set PD2 to PD7 as output
        dp.PORTD
            .portd
            .write(|w| unsafe { w.bits((input_value as u8) << 2) });
        // Set PB0 to PB3 and PB5
        dp.PORTB
            .portb
            .write(|w| unsafe { w.bits((input_value >> 6) as u8 | 0b0001_0000) });
    }
}

#[avr_device::interrupt(atmega328p)]
fn PCINT1() {
    // Notify of a change in alpha
    unsafe { FLAG = true };
}
