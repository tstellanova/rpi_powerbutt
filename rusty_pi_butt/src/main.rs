extern crate rust_pigpio;

use std::thread::sleep;
use std::time::Duration;

// We use pigpio to generate and monitor signals on GPIO pins
use rust_pigpio::*;
use rust_pigpio::pwm::*;


const LED_GPIO_PIN: u32 = 18;
const PWM_FULL_RANGE: u32 = 1000;
const LED_PWM_FREQ_HZ: u32 = 2000;
const FADE_STEP_DELAY_MS: u64 = 100;
const FADE_STEPS: u32 = 25;
const FADE_STEP_VAL: u32 = (PWM_FULL_RANGE / FADE_STEPS);

fn led_on(duration_secs: u64) {
  // the LED safety switch I'm using is already tied to +VDC: set GPIO pin low to turn on
  write(LED_GPIO_PIN, OFF).unwrap();
  println!("Bright...");
  sleep(Duration::from_secs(duration_secs));
}

fn led_off(duration_secs: u64) {
  write(LED_GPIO_PIN, ON).unwrap();
  println!("Dark...");
  sleep(Duration::from_secs(duration_secs));
}

fn fade_led_down(pin: u32 ) {
  set_pwm_frequency(pin, LED_PWM_FREQ_HZ).unwrap(); 
  set_pwm_range(pin, PWM_FULL_RANGE).unwrap(); // Set range to 1000. 1 range = 2 us;

  println!("Fade down...");
  for x in 0..FADE_STEPS{
    let duty_cycle = x * FADE_STEP_VAL;
    //println!("duty_cycle: {}", duty_cycle);
    
    pwm(pin, duty_cycle).unwrap(); 
    //TODO hardware_pwm doesn't seem to work.  Check docs, impl
    //hardware_pwm(pin, LED_PWM_FREQ_HZ, duty_cycle).unwrap();
    sleep(Duration::from_millis(FADE_STEP_DELAY_MS))
  }
}

fn fade_led_up(pin: u32) {
  set_pwm_frequency(pin, LED_PWM_FREQ_HZ).unwrap();
  set_pwm_range(pin, PWM_FULL_RANGE).unwrap(); // Set range to 1000. 1 range = 2 us;

  println!("Fade up...");

  for x in 0..FADE_STEPS{
    let duty_cycle = PWM_FULL_RANGE - x * FADE_STEP_VAL;
    //println!("duty_cycle: {}", duty_cycle);

    pwm(pin, duty_cycle).unwrap();  
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

fn led_fade_cycle(count: u32) {
  for x in 0..count {
    fade_led_up(LED_GPIO_PIN);
    fade_led_down(LED_GPIO_PIN);
  }
}

fn main() {
  println!("Initialized pigpio. Version: {}", initialize().unwrap());

  set_mode(LED_GPIO_PIN,  OUTPUT).unwrap();

  led_on(1);
  led_off(1);

  led_fade_cycle(5);

  terminate();

}
