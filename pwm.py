#!/usr/bin/env python

# start pigpiod with `sudo pigpiod` before running

import os
import time
import wiringpi
import pigpio
import RPi.GPIO as rpi_gpio

POWERUP_PIN=1

def power_off():
  print "power off"
  wiringpi.digitalWrite(POWERUP_PIN,0) 
  time.sleep(2)

def power_on():
  print "power on..."
  wiringpi.digitalWrite(POWERUP_PIN,1)
  time.sleep(1)

def power_cycle():
  power_off()
  power_on()

def int_callback(channel):  
    print "rising edge detected on 17"  
    
def main():
  # wiringpi.wiringPiSetup() 
  
  # setup the pin that will power the board
  # wiringpi.pinMode(POWERUP_PIN,1) # Set pin to 1 ( OUTPUT )

  # power_cycle()
  # time.sleep(3)


  rpi_gpio.setmode(rpi_gpio.BCM)  
  # GPIO 17 set up as an input, pulled down, connected to 3V3 on button press  
  rpi_gpio.setup(17, rpi_gpio.IN, pull_up_down=rpi_gpio.PUD_DOWN)
  rpi_gpio.add_event_detect(17, rpi_gpio.RISING, callback=int_callback, bouncetime=300)  
  
  pigpi = pigpio.pi()             # exit script if no connection
  if not pigpi.connected:
     print "no pigpio"
     exit()

  out_pin = 18
  pwm_freq = 1000
  hold_time = 0.025
  
  # black
  pigpi.hardware_PWM(out_pin, pwm_freq, 1000000)
  time.sleep(1)
  
  while (True):
    for i in range(99,0,-1):
        cycle = i*10000
        print cycle
        pigpi.hardware_PWM(out_pin, pwm_freq, cycle)
        time.sleep(hold_time)

    for i in range(1,99,1):
        cycle = i*10000
        print cycle
        pigpi.hardware_PWM(out_pin, pwm_freq, cycle)
        time.sleep(hold_time)


if __name__ == "__main__":
    main()
