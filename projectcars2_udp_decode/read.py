import socket
import struct 

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.bind(("192.168.1.255", 5606))

while 1:
	data, something = sock.recvfrom(10000)
	if len(data) != 559:
		continue
	print(chr(27) + "[2J")
	print(len(data))
	print(' '.join((f"{hex(x)[2:]:>2}" + ((i % 50 == 0 and '\n') or '') for i,x in enumerate(data))))
	decoded = {
		"throttle": data[13],
		"brake": data[14],
		"clutch": data[16],
		"oil_temp": data[18],
	}
	print(decoded)

