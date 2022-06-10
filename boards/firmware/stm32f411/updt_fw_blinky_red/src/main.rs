#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;

extern crate stm32f4xx_hal as mcu;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use mcu::delay::Delay;
use mcu::gpio;
use mcu::gpio::gpiod::PD14;
use mcu::prelude::*;
use mcu::stm32;
// use panic_probe as _;

#[cfg(feature = "defmt")]
use defmt_rtt as _; // global logger

use rustBoot_hal::stm::stm32f411::FlashWriterEraser;
use rustBoot_update::update::{update_flash::FlashUpdater, UpdateInterface};

struct Leds {
    red: PD14<gpio::Output<gpio::PushPull>>,
}

#[entry]
fn main() -> ! {
    if let (Some(peri), Some(cortex_peri)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let rcc = peri.RCC.constrain();
        let clocks1 = rcc.cfgr.sysclk(84.mhz()).freeze();
        let mut delay = Delay::new(cortex_peri.SYST, &clocks1);

        // GPIO Initialization
        let gpiod = peri.GPIOD.split();
        let mut leds = Leds {
            red: gpiod.pd14.into_push_pull_output(),
        };
        let flash1 = peri.FLASH;

        let mut count = 0;
        while count < 6 {
            leds.red.toggle();
            delay.delay_ms(1000_u16);
            count = count + 1;
        }

        let flash_writer = FlashWriterEraser { nvm: flash1 };
        let updater = FlashUpdater::new(flash_writer);
        match updater.update_success() {
            Ok(_v) => {}
            Err(e) => panic!("couldnt trigger update: {}", e),
        }

        loop {
            leds.red.toggle();
            delay.delay_ms(1000_u16);
        }
    }
    loop {}
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}