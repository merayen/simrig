# Mowtech

The "MoTec"-wannabe for Raspberry Pi 4 rev B!

## Parts
- [Raspberry Pi 4 B](https://www.raspberrypi.org/products/raspberry-pi-4-model-b/)
- [RB-LCD-5](https://joy-it.net/en/products/RB-LCD-5)
- LEDs, very bright ones, 30-35 cd, transparent lens
	- [4 green LEDs](https://www.elfadistrelec.no/en/led-515nm-green-mm-wuerth-elektronik-151054gs03000/p/30074764)
		- Series resistor: 27 Ohm # (5-3.6-.6)/.03
	- [3 yellow LEDs](https://www.elfadistrelec.no/en/led-591nm-yellow-mm-everlight-electronics-383-2uyc-h2-s400/p/17501661)
		- Series resistor: 40 Ohm  # (5-2.4-.6)/.05
	- [3 red LEDs](https://www.elfadistrelec.no/en/led-631nm-red-mm-wuerth-elektronik-151054rs03000/p/30074765)
		- Series resistor: 60 Ohm # (5-2.6-.6)/.03

## Dependencies
- [Rust](https://www.rust-lang.org/tools/install)
	- Install on Raspberry Pi 4
