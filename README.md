# Racing simulator rig

Try to make a racing rig, with real analog instruments, and a Raspberry Pi 4 with an LCD to show telemetry data.

![The rig](img/overview.jpg)

## The "MowTec"
This is the device that mocks a MoTec display. These usually have a small LCD-screen with some LEDs above lighting up when engine RPM is close to redline.

Inspired from the Ken Block's Hoonitruck, where the LEDs look like a christmas tree when he redlines his vehicle.

![The MowTec](img/mowtec-rpm.gif)

Test-implementation done with Pygame. Unsure if the LEDs should be on/off, or if a better option would be to pulse width modulate them.
