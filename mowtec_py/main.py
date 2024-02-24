"""
Python implementation of the Bosch DDU9
"""
import os
os.environ["PYGAME_HIDE_SUPPORT_PROMPT"] = "1"
import pygame

WIDTH = 720
HEIGHT = 512

pygame.init()

running = True
screen = pygame.display.set_mode((WIDTH,HEIGHT))
clock = pygame.time.Clock()

def pos(x,y):
	return (x*WIDTH, y*HEIGHT)


def draw_rpm_bar(n, n_max):
	pygame.draw.rect(screen, "grey", (*pos(.01,.01,), *pos(.98,.1)), width=1)
	pygame.draw.rect(screen, (127,100,0), (*pos(.02,.05,), *pos(.96,.05)), width=1000)


while running:
	event = pygame.event.poll()
	if event.type in (pygame.QUIT, pygame.KEYDOWN):
		pygame.quit()
		break

	screen.fill('black')
	pygame.draw.rect(screen, "white", (*pos(0,0), *pos(1, 1)), width=1)
	draw_rpm_bar(6.8, 8)

	pygame.display.flip()
	clock.tick(60)
