extern crate rust_pigpio;

use std::thread::sleep;

use rust_pigpio::*;
use rust_pigpio::pwm::*;


const LED_GPIO_PIN: u32 = 18;
const PWM_FULL_RANGE: u32 = 1000;
const LED_PWM_FREQ_HZ: u32 = 2000;
const FADE_STEP_DELAY_MS: u64 = 100;
const FADE_STEPS: u32 = 25;
const FADE_STEP_VAL: u32 = (PWM_FULL_RANGE / FADE_STEPS);

fn led_on(duration_secs: u64) {
  write(LED_GPIO_PIN, OFF).unwrap();
  println!("Bright..");
  sleep(std::time::Duration::from_secs(duration_secs));
}

fn led_off(duration_secs: u64) {
  write(LED_GPIO_PIN, ON).unwrap();
  println!("Dark...");
  sleep(std::time::Duration::from_secs(duration_secs));
}

fn fade_led_down(pin: u32 ) {
    set_pwm_frequency(pin, LED_PWM_FREQ_HZ).unwrap(); 
    set_pwm_range(pin, PWM_FULL_RANGE).unwrap(); // Set range to 1000. 1 range = 2 us;

  for x in 0..FADE_STEPS{
    let duty_cycle = x * FADE_STEP_VAL;
    //println!("duty_cycle: {}", duty_cycle);
    
    pwm(pin, duty_cycle).unwrap(); 
    //hardware_pwm(pin, LED_PWM_FREQ_HZ, duty_cycle).unwrap();
    sleep(std::time::Duration::from_millis(FADE_STEP_DELAY_MS))
  }
}

fn fade_led_up(pin: u32) {
  set_pwm_frequency(pin, LED_PWM_FREQ_HZ).unwrap();
  set_pwm_range(pin, PWM_FULL_RANGE).unwrap(); // Set range to 1000. 1 range = 2 us;

  for x in 0..FADE_STEPS{
    let duty_cycle = PWM_FULL_RANGE - x * FADE_STEP_VAL;
    //println!("duty_cycle: {}", duty_cycle);

    pwm(pin, duty_cycle).unwrap();  
    //hardware_pwm(pin, LED_PWM_FREQ_HZ, duty_cycle).unwrap();
    sleep(std::time::Duration::from_millis(FADE_STEP_DELAY_MS))
  }

}

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
