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

	p = kmt_value.render(str(round(kmt/2)*2), True, "white")
	r = pos(.72, .3)
	surface.blit(p, (r[0] - p.get_size()[0], r[1]))


boost_titles = [
	pygame.font.SysFont("Arial", font_size(21)).render("boost_LH", True, "grey", "black"),
	pygame.font.SysFont("Arial", font_size(21)).render("boost_RH", True, "grey", "black"),
]
bar_digit = pygame.font.SysFont("Arial", font_size(16))

def draw_boost(x, y, n, index):
	pygame.draw.rect(surface, "grey", (*pos(x, y), *pos(.4, .18)), width=size(1))
	surface.blit(boost_titles[index], pos(x + .12, y-.02))

	# Boost scale
	for i in range(4):
		v = 0.12*(i/1.5) + .1
		pygame.draw.line(surface, "grey", [*pos(x+v, y+.05)], [*pos(x+v, y+.15)], True)
		surface.blit(bar_digit.render(str(i / 2), True, "grey", (0,0,0)), pos(x + v - .02, y + .05))

	# Boost bar
	pygame.draw.rect(surface, "white", (*pos(x + .02, y+.1), *pos(.08 + n/4, .05)), width=size(10))

while running:
	event = pygame.event.poll()
	if event.type in (pygame.QUIT, pygame.KEYDOWN):
		pygame.quit()
		break

	surface.fill('black')
	pygame.draw.rect(surface, "white", (*pos(0,0), *pos(1, 1)), width=1)

	draw_rpm_bar(1.8, 8)
	draw_speed(210)
	draw_boost(.02, .8, 1, 0)
	draw_boost(.58, .8, 1, 1)

	pygame.transform.scale2x(surface, screen)
	#screen.blit(surface, (0,0), )

	pygame.display.flip()
	clock.tick(60)
