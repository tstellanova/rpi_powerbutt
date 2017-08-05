#!/usr/bin/env python

# start pigpiod with `sudo pigpiod` before running

import os
import time
import pigpio
import RPi.GPIO as rpi_gpio
import subprocess

def restart_pi():
  command = "/usr/bin/sudo /sbin/shutdown -r now"
  process = subprocess.Popen(command.split(), stdout=subprocess.PIPE)
  output = process.communicate()[0]
  print output

def shutdown_pi():
  command = "/usr/bin/sudo /sbin/shutdown -h now"
  process = subprocess.Popen(command.split(), stdout=subprocess.PIPE)
  output = process.communicate()[0]
  print output

def int_callback(channel):  
    print "rising edge detected on 17-- shutdown..."  
    shutdown_pi()

def main():

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
