from merayen import Draw
from math import *

rpm = 0.5 #  visible RPM LEDs, value: 0.0 - 1.0. Usually rpm should raise from 0.0 when engine is close to rev limiter, and outputting max watt

WIDTH = .2  # Width of the LED-train
LEDS = [
	(0,1,0),
	(0,1,0),
	(0,1,0),
	(0,1,0),
	(1,1,0),
	(1,1,0),
	(1,1,0),
	(1,0,0),
	(1,0,0),
	(1,0,0),
]

START_RPM = 0.6  # When the left LED is the center
STOP_RPM = 0.75   # When the right LED is the center

with Draw() as p:
	while p.run():

		#rpm = (sin(p.time*2)/2+.5)*.3 + .7
		#rpm = sin(p.time)/2+.5
		rpm += (((p.time * .2) % 1)*.3 + .50 - rpm) * p.delta * 7

		# Circles
		#print()
		meter_rpm = min(1, (rpm - START_RPM) / (STOP_RPM - START_RPM))
		for i in range(len(LEDS)):
			a = (-i/len(LEDS)) * (100/360 * 2 * pi) - pi*0.75
			x = .5 + sin(a)*.6
			y = .6 + cos(a)*.5

			# Calculate brightness of this LED: i
			brightness = int(rpm * len(LEDS)) == i  # Single LED
			brightness = 1/(1+abs(meter_rpm * (len(LEDS)-1) - i)**6)**.2  # Nice, with fall-off on both side (requires PWM or analog output from IC)
			#brightness = 1/(1+abs(meter_rpm * (len(LEDS)-1) - i)**6)**.2 > .4  # Same, but no fall-off on sides (on/off LEDs)

			color = tuple(x * brightness for x in LEDS[i])

			#print(i,"*"*int(brightness*100))
			assert brightness>=0 and brightness<=1,brightness
			p.color(*color)
			p.fcircle(x,y,10)

		p.color(.1,.1,.1)
		p.rect(.1,.3,.9,.9)
		p.color(1,1,1)
		p.text(.1,.3, "Raspberry Pi 4!")
		p.text(.4,.8, f"RPM: {int(rpm*10000)}")

		p.circle(.2,.3,.3)
		viser = 2 * pi * .75 * (1-rpm) + pi/4
		p.line(.5,.6, .5 + sin(viser) * .28, .6 + cos(viser) * .28, 2)

		for i in range(0,11):
			tegn = 2 * pi * .75 * (1 - (i/10)) + pi/4
			p.text(.49 + sin(tegn) * .28, .6 + cos(tegn) * .28, str(i))
