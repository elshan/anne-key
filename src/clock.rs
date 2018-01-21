pub fn init_clock(p: &super::init::Peripherals) {
    p.USB.usb_cntr.modify(|_, w| w.pdwn().clear_bit());

    p.FLASH.acr.modify(|_, w| w.acc64().set_bit());
    p.FLASH.acr.modify(|_, w| w.prften().set_bit());
    p.FLASH.acr.modify(|_, w| w.latency().set_bit());

    p.RCC.cr.modify(|_, w| w.hseon().set_bit());
    while p.RCC.cr.read().hserdy().bit_is_clear() {}

    p.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());
    p.RCC.apb1enr.modify(|_, w| w.pwren().set_bit());

    p.PWR.cr.modify(|_, w| {
        w.lprun().clear_bit();
        unsafe { w.vos().bits(0b01) }
    });
    while p.PWR.csr.read().vosf().bit_is_set() {}

    p.RCC.cfgr.modify(|_, w| unsafe {
        w.ppre1().bits(0b100)
         .ppre2().bits(0b100)
         .pllmul().bits(0b0010)
         .plldiv().bits(0b10)
         .pllsrc().set_bit()
    });

    p.RCC.cr.modify(|_, w| w.pllon().set_bit());
    while p.RCC.cr.read().pllrdy().bit_is_clear() {}

    p.RCC.cfgr.modify(|_, w| unsafe { w.sw().bits(0b11) });
    while p.RCC.cfgr.read().sws().bits() != 0b11 {}

    p.RCC.cr.modify(|_, w| w.msion().clear_bit());

    p.RCC.ahbenr.modify(|_, w|
        w.gpiopaen().set_bit()
         .gpiopben().set_bit());
}
