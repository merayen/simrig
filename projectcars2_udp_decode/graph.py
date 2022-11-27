import json
with open("output.json") as f:
	data = json.load(f)

timestamps = [x["time"] for x in data]
rpm = [x["rpm"] for x in data]
wheel_rotations = [x["wheel_rotations"] for x in data[:]]
rotation = [x["rotation"] for x in data]
location = [x["location"] for x in data]

import numpy as np

def unmodulo(d):
	last = d[0]
	offset = 0
	for v in d:
		diff = v - last

unmodulo([x[0] for x in rotation])

a = [x["rotation"][0] for x in data]
#a *= np.random.random(len(a))

#a += np.sin(np.linspace(0,np.pi*2*500, len(a))) * 100
from scipy.fft import rfft
a = rfft(a)

a = [(x.real**2+x.imag**2)**.5 for x in a]

a = np.log(a)/np.log(2)

time_diff = [0]*(len(timestamps)-1)
last = timestamps[0]
for i,x in enumerate(timestamps[1:]):
	time_diff[i] = x - last
	last = x

print(time_diff[:10])

import pylab as pl
fig, axs = pl.subplots(1)
axs.plot(time_diff, label="time_diff")
axs.legend()
pl.waitforbuttonpress()
pl.close()
