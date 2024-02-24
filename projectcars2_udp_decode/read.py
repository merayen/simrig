import socket
import struct 
import time
import json

output = []

import sys
assert sys.argv[1].endswith(".txt")

with open(sys.argv[1], "rb") as f:
	while 1:
		length = f.read(8)
		if not length:
			break

		assert length.isdigit()

		length = int(length)

		time_offset = f.read(8)
		assert time_offset
		assert time_offset.isdigit()
		time_offset = int(time_offset)

		data = f.read(length)
		if len(data) != 559:
			continue
		#print(' '.join((((i % 50 == 0 and '\n') or '') + f"{hex(x)[2:]:>2}" for i,x in enumerate(data))))
		decoded = {
			"time": time_offset / 1000,
			"throttle": data[13],
			"brake": data[14],
			"clutch": data[16],
			"oil_temp": data[18],
			"speed": struct.unpack("f", data[36:40]),
			"rpm": struct.unpack("H", data[40:42]),
			"torque": struct.unpack("f", data[364:368])[0],
			"gear": data[45] - 96 if data[45] != 111 else -1,
			"location": struct.unpack("fff", data[542:554]),
			"rotation": struct.unpack("fff", data[52:64]),
			"wheel_rotations": struct.unpack("ffff", data[280:296]),
		}
		output.append(decoded)

with open(f"{sys.argv[1].rsplit('.', maxsplit=1)[0]}.json", "w") as f:
	f.write("[\n")
	for i,x in enumerate(output):
		f.write(f"\t{json.dumps(x)}")
		if i+1 < len(output):
			f.write(",")
		f.write("\n")
	f.write("]")
