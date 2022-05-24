#![no_main]
#![no_std]

use stm32_project as _;
// use panic_halt as _;

#[rtic::app(device = stm32l4xx_hal::stm32,
dispatchers = [DFSDM1],
peripherals = true,
)
]

mod app {
    use defmt::export::panic;
    use systick_monotonic::fugit::{Duration, RateExtU32};
    use systick_monotonic::{Systick};
    use stm32l4xx_hal::prelude::_stm32l4_hal_RccExt;
    use stm32l4xx_hal::gpio::{GpioExt, Output, PushPull};
    use stm32l4xx_hal::gpio::PB3;
    use stm32l4xx_hal::flash::FlashExt;
    use stm32l4xx_hal::prelude::_stm32l4_hal_PwrExt;
    // use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {
    }

    #[local]
    struct Local {
        led: PB3<Output<PushPull>>,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = Systick<1000>;

    #[init]
    fn init(c: init::Context) -> (Shared, Local, init::Monotonics) {
        // This is the `cortex_m::Peripherals` struct without the SysTick which RTIC has taken ownership of.
        let cp = c.core;
        // Device access (Peripheral Access Crate)
        let pac = c.device;

        // Use the HAL to get a pin to control.
        let mut rcc = pac.RCC.constrain();
        let mut gpiob = pac.GPIOB.split(&mut rcc.ahb2);
        let mut flash = pac.FLASH.constrain();
        let mut pwr = pac.PWR.constrain(&mut rcc.apb1r1);

        let _clocks = rcc
            .cfgr
            .sysclk(80.MHz())
            .pclk1(80.MHz())
            .pclk2(80.MHz())
            .freeze(&mut flash.acr, &mut pwr);

        let mut led = gpiob
            .pb3
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

        led.set_low();

        // rtt_init_print!(); // You may prefer to initialize another way

        let mono = Systick::new(cp.SYST, 80_000_000);

        // Start the blinky task!
        blink_led::spawn().unwrap();

        (
            Shared {  },
            Local { led },
            init::Monotonics(mono),
        )
    }

    #[task(local = [led, flag: bool = false])]
    fn blink_led(cx: blink_led::Context) {
        // Extract the LED
        let led = cx.local.led;
        let flag = cx.local.flag;

        if *flag == false {
            led.set_low();
            defmt::println!("LED ON");
        } else {
            led.set_high();
            defmt::println!("LED OFF");
        }

        blink_led::spawn_after(Duration::<u64, 1, 1000>::from_ticks(500)).unwrap();

        *flag = !*flag;
        // panic!("flag = {}", *flag);
        // assert!(false);
    }

}