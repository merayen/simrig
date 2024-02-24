"""
Python implementation of the Bosch DDU9
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
clock = pygame.time.Clock()

lolnumber = 500


def pos(x,y=None):
	if y is not None:
		return (x*WIDTH, y*HEIGHT)
	return x*((WIDTH+HEIGHT)/2)


def font_size(a):
	return round(a*((WIDTH+HEIGHT)/2) / lolnumber)

def size(x):
	return round(x*((WIDTH+HEIGHT)/2) / lolnumber)


rpm_title = pygame.font.SysFont("Arial", font_size(12)).render("rpm", True, (127,100,0), (0,0,0))


def draw_rpm_bar(n, n_max):
	# RPM lines
	rpm_digit = pygame.font.SysFont("Arial", font_size(12))
	for i in range(n_max):
		v = 1.1*(i/n_max) + .02
		pygame.draw.line(screen, "grey", [*pos(v, .05)], [*pos(v, .1)], True)
		screen.blit(rpm_digit.render(str(i+1), True, (127,100,0), (0,0,0)), pos(v - .005, .05))

	pygame.draw.rect(screen, "grey", (*pos(.01,.01,), *pos(.98, .1)), width=size(1))

	# RPM moving bar
	pygame.draw.rect(screen, (127,100,0), (*pos(.02,.05,), *pos(1.1*(max(0, min(n, n_max)-1)/n_max),.05)), width=1000)

	screen.blit(rpm_title, pos(.5, .01))


while running:
	event = pygame.event.poll()
	if event.type in (pygame.QUIT, pygame.KEYDOWN):
		pygame.quit()
		break

	screen.fill('black')
	pygame.draw.rect(screen, "white", (*pos(0,0), *pos(1, 1)), width=1)
	draw_rpm_bar(4, 8)

	pygame.display.flip()
	clock.tick(60)
