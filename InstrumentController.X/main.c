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

unsigned long ticks = 0;
unsigned long time_ms = 0;

unsigned char current_TMR1H = 0;
unsigned char current_TMR1L = 0;

struct State {
	unsigned int rpm; // 0 = 0 RPM, 255 = 10'000 rpm
	unsigned char led_rpm; // 0 = off, 1 = first green LED dimly lit, 255 = right-most LED lit
} state;

void __interrupt () my_little_interrupting_pony() {
	if (PIR1bits.TMR1IF) {
		//ticks++;
		LATEbits.LATE0 ^= 1;
		TMR1Lbits.TMR1L = current_TMR1L;
		TMR1Hbits.TMR1H = current_TMR1H;
		PIR1bits.TMR1IF = 0;
	}
	//if (PIR1bits.TMR2IF) {
	//	PIR1bits.TMR2IF = 0;
	//}
}

//void time_update(void) {
//	unsigned long something = ticks;
//	ticks = 0; // May loose a tick if interrupted around this part, but whatever
//	if (something > 0) {
//		time_ms += something * 8;
//		state.rpm += 1;
//	}
//}

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

	TRISA = 0;
	TRISC = 0;
	TRISD &= 255 - 1;
	TRISE &= 255 - 1;

	// 32 MHz action
	OSCCONbits.SCS = 0;
	OSCCONbits.IRCF0 = 1;
	OSCCONbits.IRCF1 = 1;
	OSCCONbits.IRCF2 = 1;
	OSCCONbits.IRCF3 = 1;

	// Wait for 32Mhz action
	while (OSCSTATbits.HFIOFR == 0 || OSCSTATbits.HFIOFL == 0 || OSCSTATbits.HFIOFR == 0 || OSCSTATbits.PLLR == 1);

	int down_step = 0;
	int led_step = 0;

	LATA = 0;
	LATC = 0;

	// Main action
	while (1) {
		//time_update();
		led_step++;
		if (led_step > 1000) {
			//if (LATA == 0 && LATC == 0) LATA = 1; else LATA *= 2;
			//if (LATA == 0 && LATC == 0) LATC = 1; else LATC *= 2;
			LATA = 0;
			LATC = 0;
			LATD ^= 1;
			led_step = 0;
		}

		down_step++;
		if (down_step > 15) {
			state.rpm++;
			down_step = 0;
		}
		if (state.rpm > 10000)
			state.rpm = 500;

		if (state.rpm >= 500) {
			unsigned int new = 65535 - (unsigned int)((unsigned long)7386363 / (unsigned long)state.rpm);
			7386363.074999999; // 7575757 / (8/7.8)
			current_TMR1L = (unsigned char)(new % 256);
			current_TMR1H = (unsigned char)(new >> 8);
			T1CONbits.TMR1ON = 1;
			//PIR1bits.TMR1IF = 0;
		} else {
			T1CONbits.TMR1ON = 0;
		}
	}
}
