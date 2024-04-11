#![no_std]
#![no_main]

use panic_halt as _;
use filters::LowPassFilter;

const CUTOFF_FREQUENCY: u16 = 4;

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
    // Set PB0 to PB3 as output
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
    });

    let mut filter = LowPassFilter::new(CUTOFF_FREQUENCY);
    let mut voltage: u16;

    loop {
        // Enable the latch takeover
        dp.PORTB.portb.modify(|_, w| w.pb4().set_bit());
        // Wait for the conversion to complete
        while dp.ADC.adcsra.read().adsc().bit_is_set() {}
        // Read the result
        voltage = dp.ADC.adc.read().bits();
        // Start the next conversion
        dp.ADC
            .adcsra
            .write(|w| w.aden().set_bit().adsc().set_bit().adps().prescaler_2());
        // Apply the filter
        filter.low_pass(&mut voltage);
        // Shift the bits to the left by 2
        voltage <<= 2;
        // prepare the bits for portb and portd
        let portd_byte = voltage as u8;
        let portb_byte = (voltage >> 8) as u8;
        // Disable the latch takeover
        dp.PORTB.portb.modify(|_, w| w.pb4().clear_bit());
        // Set PD2 to PD7 as output
        dp.PORTD.portd.write(|w| unsafe { w.bits(portd_byte) });
        // Set PB0 to PB3 as output
        dp.PORTB.portb.write(|w| unsafe { w.bits(portb_byte) });
    }
}
