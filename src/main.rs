#![no_std]
#![no_main]

use arduino_hal::simple_pwm::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    // Digital pin 5 is connected to a LED and a resistor in series
    let mut blue_pwm = pins.d9.into_output().into_pwm(&timer1);
    let mut green_pwm = pins.d5.into_output().into_pwm(&timer0);
    let mut red_pwm = pins.d6.into_output().into_pwm(&timer0);

    blue_pwm.enable();
    green_pwm.enable();
    red_pwm.enable();

    loop {
        let mut blue_value = 255;
        let mut green_value = 0;
        let mut red_value = 0;
        for _ in 0..255 {
            blue_value -= 1;
            green_value += 1;
            blue_pwm.set_duty(blue_value);
            green_pwm.set_duty(green_value);
            arduino_hal::delay_ms(10);
        }
        blue_value = 0;
        green_value = 255;
        red_value = 0;
        for _ in 0..255 {
            green_value -= 1;
            red_value += 1;
            green_pwm.set_duty(green_value);
            red_pwm.set_duty(red_value);
            arduino_hal::delay_ms(10);
        }
        blue_value = 0;
        green_value = 0;
        red_value = 255;
        for _ in 0..255 {
            red_value -= 1;
            blue_value += 1;
            red_pwm.set_duty(red_value);
            blue_pwm.set_duty(blue_value);
            arduino_hal::delay_ms(10);
        }
    }
}
