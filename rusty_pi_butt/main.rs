extern crate rust-pigpio;


const LED_GPIO_PIN: u32 = 18;

fn main() {
  // set the pwm value
  hardware_pwm(LED_GPIO_PIN, 1000, 50);
  //hardware_pwm(gpio: u32, frequency: u32, dutycycle: u32)
  println!("Yello!");
}
