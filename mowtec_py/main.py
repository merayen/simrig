"""
Python implementation of the Bosch DDU9

Scales 2x, giving a bit more pixelated design.
"""
import os
os.environ["PYGAME_HIDE_SUPPORT_PROMPT"] = "1"
import pygame

WIDTH = 720
HEIGHT = 512

pygame.init()
pygame.font.init()

running = True
screen = pygame.display.set_mode((WIDTH,HEIGHT))
surface = pygame.Surface(size=(int(WIDTH/2), int(HEIGHT/2)))
clock = pygame.time.Clock()

lolnumber = 500


def pos(x,y=None):
	if y is not None:
		return (x*WIDTH/2, y*HEIGHT/2)
	return x*((WIDTH+HEIGHT)/2/2)


def font_size(a):
	return round(a*((WIDTH+HEIGHT)/2/2) / lolnumber)

def size(x):
	return round(x*((WIDTH+HEIGHT)/2/2) / lolnumber)


rpm_title = pygame.font.SysFont("Arial", font_size(12)).render("rpm", True, (127,100,0), (0,0,0))


def draw_rpm_bar(n, n_max):
	# RPM lines
	rpm_digit = pygame.font.SysFont("Arial", font_size(12))
	for i in range(n_max):
		v = 1.1*(i/n_max) + .02
		pygame.draw.line(surface, "grey", [*pos(v, .05)], [*pos(v, .1)], True)
		surface.blit(rpm_digit.render(str(i+1), True, (127,100,0), (0,0,0)), pos(v - .005, .05))

	pygame.draw.rect(surface, "grey", (*pos(.01,.01,), *pos(.98, .1)), width=size(1))

	# RPM moving bar
	pygame.draw.rect(surface, (127,100,0), (*pos(.02,.05,), *pos(1.1*(max(0, min(n, n_max)-1)/n_max),.05)), width=1000)

	surface.blit(rpm_title, pos(.5, .01))


kmt_title = pygame.font.SysFont("Arial", font_size(32)).render("km/h", True, "grey")
kmt_value = pygame.font.SysFont("Arial", font_size(128))

def draw_speed(kmt):
	pygame.draw.rect(surface, "grey", (*pos(.2, .15), *pos(.6, .5)), width=size(1))
	surface.blit(kmt_title, pos(.445, .2))
	surface.blit(kmt_value.render(str(round(kmt/2)*2), True, "white"), pos(.322, .3))


while running:
	event = pygame.event.poll()
	if event.type in (pygame.QUIT, pygame.KEYDOWN):
		pygame.quit()
		break

	surface.fill('black')
	pygame.draw.rect(surface, "white", (*pos(0,0), *pos(1, 1)), width=1)

	draw_rpm_bar(1.8, 8)
	draw_speed(0)

	pygame.transform.scale2x(surface, screen)
	#screen.blit(surface, (0,0), )

	pygame.display.flip()
	clock.tick(60)
