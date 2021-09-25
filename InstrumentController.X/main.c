/*
 * File:   main.c
 * Author: merayen
 */

// CONFIG1
#pragma config FOSC = INTOSC    // Oscillator Selection (INTOSC oscillator: I/O function on CLKIN pin)
#pragma config WDTE = OFF       // Watchdog Timer Enable (WDT disabled)
#pragma config PWRTE = OFF      // Power-up Timer Enable (PWRT enabled)
#pragma config MCLRE = ON       // MCLR Pin Function Select (MCLR/VPP pin function is MCLR)
#pragma config CP = OFF         // Flash Program Memory Code Protection (Program memory code protection is disabled)
#pragma config CPD = OFF        // Data Memory Code Protection (Data memory code protection is disabled)
#pragma config BOREN = ON       // Brown-out Reset Enable (Brown-out Reset enabled)
#pragma config CLKOUTEN = OFF   // Clock Out Enable (CLKOUT function is disabled. I/O or oscillator function on the CLKOUT pin)
#pragma config IESO = OFF        // Internal/External Switchover (Internal/External Switchover mode is enabled)
#pragma config FCMEN = OFF       // Fail-Safe Clock Monitor Enable (Fail-Safe Clock Monitor is enabled)

// CONFIG2
#pragma config WRT = OFF        // Flash Memory Self-Write Protection (Write protection off)
#pragma config VCAPEN = OFF     // Voltage Regulator Capacitor Enable bit (Vcap functionality is disabled on RA6.)
#pragma config PLLEN = ON       // PLL Enable (4x PLL enabled)
#pragma config STVREN = ON      // Stack Overflow/Underflow Reset Enable (Stack Overflow or Underflow will cause a Reset)
#pragma config BORV = LO        // Brown-out Reset Voltage Selection (Brown-out Reset Voltage (Vbor), low trip point selected.)
#pragma config LPBOR = OFF      // Low Power Brown-Out Reset Enable Bit (Low power brown-out is disabled)
#pragma config LVP = ON        // Low-Voltage Programming Enable (Low-voltage programming enabled)
#include <xc.h>

#define LED_COUNT 18
#define LED_SOCKETS 11 // Physical LEDs in total (they are RGB)
#define LED_WIDTH 5
#define LED_PWM_RESOLUTION 4

unsigned char current_TMR1H = 0;
unsigned char current_TMR1L = 0;

// These two parameters should be retrieved from SPI, engine specs
short led_start = 6500; // Where LEDs start to light on left side (green LEDs)
short led_stop = 7800; // Max RPM where LEDs on the right side won't go any further

struct State {
	unsigned int rpm; // 0 = 0 RPM, 255 = 10'000 rpm
	unsigned char led_rpm; // 0 = off, 1 = first green LED dimly lit, 255 = right-most LED lit
} state;

void __interrupt () my_little_interrupting_pony() {
	if (PIR1bits.TMR1IF) {
		LATEbits.LATE0 ^= 1;
		TMR1Lbits.TMR1L = current_TMR1L;
		TMR1Hbits.TMR1H = current_TMR1H;
		PIR1bits.TMR1IF = 0;
	}
}

unsigned char led_power[LED_COUNT];
unsigned char led_power_position = 0;

void update_leds() {
	// Position can be between 0 and 11 (not inclusive), the center of the RPM LED
	short position = ((short)state.rpm - (short)led_start) / ((short)(led_stop - led_start) / (short)LED_SOCKETS); // Position of the center LED

	if (position > LED_SOCKETS - 1) position = LED_SOCKETS - 1;

	// If totally outside, nothing to do
	if (position < -LED_WIDTH) return;

	unsigned char leds[LED_SOCKETS];

	for (char i = 0; i < LED_SOCKETS; i++)
		leds[i] = i*14; // 256/18

	// Middle LED
	//for (char i = -LED_WIDTH; i < LED_SOCKETS; i++) {
	//	int offset = abs(i - (char)position);
	//	if (i >= 0 && i < LED_SOCKETS) {
	//		if (offset == 0) leds[i] = 255; else leds[i] = 0;
	//	}
	//}

	for (char i = 0; i < LED_SOCKETS; i++) {
		if (position >= i && position - LED_WIDTH < i)
			leds[i] = 255;
		else
			leds[i] = 0;
	}

	// Apply PWM values on the different LEDs (0 to 255)

	// Green LEDs
	// pyx 4 f'led_power[{i}] = leds[{i}];\n'
	led_power[0] = leds[0];
	led_power[1] = leds[1];
	led_power[2] = leds[2];
	led_power[3] = leds[3];

	// Red LEDs (turns yellow)
	// pyx 6 f'led_power[{i+4}] = leds[{i+4}] >> {int(1+i/1.5)};\n'
	led_power[4] = leds[4] >> 1;
	led_power[5] = leds[5] >> 1;
	led_power[6] = leds[6] >> 2;
	led_power[7] = leds[7] >> 3;
	led_power[8] = leds[8] >> 3;
	led_power[9] = leds[9] >> 4;

	// Red LEDs
	// pyx 6 f'led_power[{i+10}] = leds[{i+4}];\n'
	led_power[10] = leds[4];
	led_power[11] = leds[5];
	led_power[12] = leds[6];
	led_power[13] = leds[7];
	led_power[14] = leds[8];
	led_power[15] = leds[9];
}

void update_led_pwm() { // Meh, don't think we have enough CPU power for this
	unsigned int lat = 0;

	led_power_position += LED_PWM_RESOLUTION;

	for (int i = 0; i < LED_COUNT; i++)
		if (led_power[i] > led_power_position)
			lat += 1 << i;

	// The driver circuit inverts, so 0 means "LED on"
	lat ^= 4294967295; // 2**32-1

	LATA = (unsigned char)lat;
	LATC = *(((unsigned char*)&lat) + 1);
	//LATDbits.LATD0 = lat & 65536 == 1;
	//LATDbits.LATD1 = lat & 131072 == 1;
}

void main(void) {
	// TIMER1 configuration
	T1CONbits.TMR1CS = 0;
	T1CONbits.T1CKPS = 3;
	T1CONbits.TMR1ON = 1;

	// Reset TIMER1 state
	TMR1Lbits.TMR1L = 0;
	TMR1Hbits.TMR1H = 0;

	// Timer1 interrupt when rollover
	PIR1bits.TMR1IF = 0;
	PIE1bits.TMR1IE = 1;
	INTCONbits.PEIE = 1;
	INTCONbits.GIE = 1;

	// TIMER2 configuration
	//T2CONbits.TMR2ON = 1;
	//T2CONbits.T2OUTPS = 1;
	//T2CONbits.T2CKPS = 0;
	//PIR1bits.TMR2IF = 0;
	//PIE1bits.TMR2IE = 1;

	// 32 MHz action
	OSCCONbits.SCS = 0;
	OSCCONbits.IRCF0 = 1;
	OSCCONbits.IRCF1 = 1;
	OSCCONbits.IRCF2 = 1;
	OSCCONbits.IRCF3 = 1;

	// Wait for 32Mhz action
	while (OSCSTATbits.HFIOFR == 0 || OSCSTATbits.HFIOFL == 0 || OSCSTATbits.HFIOFR == 0 || OSCSTATbits.PLLR == 1);

	TRISA = 0;
	TRISC = 0;
	TRISD &= 255 - 1;
	TRISE &= 255 - 1;

	int tick_update_rpm = 0;
	int tick_update_tachometer = 0;
	int tick_led_update = 0;

	// Main action
	while (1) {
		tick_update_rpm++;
		if (tick_update_rpm > 100) {
			tick_update_rpm = 0;
			state.rpm++;
		}
		if (state.rpm > 7800 || state.rpm < 5000)
			state.rpm = 6000;

		tick_update_tachometer++;
		if (tick_update_tachometer > 100 && state.rpm >= 500) {
			tick_update_tachometer = 0;
			unsigned int new = 65535 - (unsigned int)((unsigned long)7386363 / (unsigned long)state.rpm);
			7386363.074999999; // 7575757 / (8/7.8)
			current_TMR1L = (unsigned char)(new % 256);
			current_TMR1H = (unsigned char)(new >> 8);
			T1CONbits.TMR1ON = 1;
			PIR1bits.TMR1IF = 0;
		} else {
			T1CONbits.TMR1ON = 0;
		}

		// RPM LEDs
		tick_led_update++;
		if (tick_led_update > 100) {
			tick_led_update = 0;
			update_leds();
		}
		update_led_pwm();
	}
}
