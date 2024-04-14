#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use filters::LowPassFilter;
use panic_halt as _;

const CUTOFF_FREQUENCY: u16 = 1;
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
    dp.EXINT.pcmsk1.write(|w| w.pcint().bits(0b0011_1110));
    dp.EXINT.pcicr.write(|w| w.pcie().bits(0b0000_0010));

    let mut filter = LowPassFilter::new(CUTOFF_FREQUENCY);
    let mut voltage: u16;

    loop {
        // Enable the latch takeover
        dp.PORTB.portb.modify(|_, w| w.pb4().set_bit());

        avr_device::interrupt::free(|_| {
            let flag = unsafe { FLAG };
            if flag {
                // Reset the flag
                unsafe {
                    FLAG = false;
                }

                // Read the alpha value and update the filter
                let alpha = ((dp.PORTC.pinc.read().bits() & 0b00111110) >> 1) + 1;
                filter.set_alpha(alpha as u16);
            }
        });

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
        // Disable the latch takeover
        dp.PORTB.portb.modify(|_, w| w.pb4().clear_bit());
        // Set PD2 to PD7 as output
        dp.PORTD
            .portd
            .write(|w| unsafe { w.bits((voltage as u8) << 2) });
        // Set PB0 to PB3 as output
        dp.PORTB
            .portb
            .write(|w| unsafe { w.bits((voltage >> 6) as u8) });
    }
}

#[avr_device::interrupt(atmega328p)]
fn PCINT1() {
    // Notify of a change in alpha
    unsafe { FLAG = true };
}
