extern crate rust_pigpio;

use std::thread::sleep;
use rust_pigpio::*;
use rust_pigpio::pwm::*;


const LED_GPIO_PIN: u32 = 18;

fn main() {
  println!("Initialized pigpio. Version: {}", initialize().unwrap());

  set_mode(LED_GPIO_PIN, OUTPUT).unwrap();
  write(LED_GPIO_PIN, ON).unwrap();
  sleep(std::time::Duration::from_secs(3));
  //write(LED_GPIO_PIN, OFF).unwrap();

  // set the pwm value
  //hardware_pwm(LED_GPIO_PIN, 1000, 50).unwrap();
  //hardware_pwm(gpio: u32, frequency: u32, dutycycle: u32)
  terminate();

}
