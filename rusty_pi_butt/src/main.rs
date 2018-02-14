extern crate rustgpio;

use rustgpio::pigpio;

use std::thread::sleep;
use std::time::Duration;

// We use pigpio to generate and monitor signals on GPIO pins
//use rust_pigpio::pwm::*;


const LED_GPIO_PIN: u32 = 18;
const PWM_FULL_RANGE: u32 = 1000;
const LED_PWM_FREQ_HZ: u32 = 2000;
const FADE_STEP_DELAY_MS: u64 = 100;
const FADE_STEPS: u32 = 25;
const FADE_STEP_VAL: u32 = (PWM_FULL_RANGE / FADE_STEPS);

fn led_on(pi: &pigpio::Pi, duration_secs: u64) {
  // the LED safety switch I'm using is already tied to +VDC: set GPIO pin low to turn on
  pi.write(LED_GPIO_PIN, pigpio::PUD_OFF);
  println!("Bright...");
  sleep(Duration::from_secs(duration_secs));
}

fn led_off(pi: &pigpio::Pi, duration_secs: u64) {
  pi.write(LED_GPIO_PIN, pigpio::PUD_UP);
  println!("Dark...");
  sleep(Duration::from_secs(duration_secs));
}

fn fade_led_down(pi: &pigpio::Pi, pin: u32 ) {
  pi.set_pwm_frequency(pin, LED_PWM_FREQ_HZ); 
  pi.set_pwm_range(pin, PWM_FULL_RANGE); // Set range to 1000. 1 range = 2 us;

  println!("Fade down...");
  for x in 0..FADE_STEPS{
    let duty_cycle = x * FADE_STEP_VAL;
    //println!("duty_cycle: {}", duty_cycle);
    
    pi.set_pwm_dutycycle(pin, duty_cycle); 
    //TODO hardware_pwm doesn't seem to work.  Check docs, impl
    //hardware_pwm(pin, LED_PWM_FREQ_HZ, duty_cycle).unwrap();
    sleep(Duration::from_millis(FADE_STEP_DELAY_MS))
  }
}

fn fade_led_up(pi: &pigpio::Pi, pin: u32) {
  pi.set_pwm_frequency(pin, LED_PWM_FREQ_HZ);
  pi.set_pwm_range(pin, PWM_FULL_RANGE); // Set range to 1000. 1 range = 2 us;

  println!("Fade up...");

  for x in 0..FADE_STEPS{
    let duty_cycle = PWM_FULL_RANGE - x * FADE_STEP_VAL;
    //println!("duty_cycle: {}", duty_cycle);

    pi.set_pwm_dutycycle(pin, duty_cycle);  
    //TODO hardware_pwm doesn't seem to work.  Check docs, impl

    //let pwm_duty = (duty_cycle / PWM_FULL_RANGE) * 1000000;
    //hardware_pwm(pin, LED_PWM_FREQ_HZ, pwm_duty).unwrap();
    sleep(Duration::from_millis(FADE_STEP_DELAY_MS))
  }

}


/*
   gpio: see description
PWMfreq: 0 (off) or 1-125000000 (125M)
PWMduty: 0 (off) to 1000000 (1M)(fully on)
*/

fn led_fade_cycle(pi: &pigpio::Pi, count: u32) {
  for _x in 0..count {
    fade_led_up(pi, LED_GPIO_PIN);
    fade_led_down(pi, LED_GPIO_PIN);
  }
}

fn main() {
  println!("Initializing...");
  let pi = pigpio::Pi::new();
  println!("Initialized pigpiod. ");

  pi.set_mode(LED_GPIO_PIN,  pigpio::OUTPUT );

  led_on(&pi, 1);
  led_off(&pi, 1);

  led_fade_cycle(&pi, 5);


}
