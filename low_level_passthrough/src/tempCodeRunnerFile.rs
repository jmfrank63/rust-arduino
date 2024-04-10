    dp.ADC.adcsra.write(|w| w.aden().set_bit().adps().prescaler_128().bits(0x97));
