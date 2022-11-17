import pygame
import argparse

class BitboardVisualizer:
    def __init__(self, position, width=11*50 + 2, height=11*50 + 2):
        pygame.init()
        self.position = position
        self.width, self.height = width, height
        self.display = pygame.display.set_mode((self.width, self.height))
        self.num_boxes = 11
        self.box_size = self.width // self.num_boxes
        self.bg_color = (150, 150, 150)
        self.line_color = (50, 50, 50)
        self.corner_color = (80, 150, 160)
        self.images = [
            pygame.transform.smoothscale(pygame.image.load('images/0.png'), (52, 52)),
            pygame.transform.smoothscale(pygame.image.load('images/1.png'), (52, 52)),
            pygame.transform.smoothscale(pygame.image.load('images/2.png'), (52, 52))
        ]


    def handle_events(self):
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                self.is_running = False
    
    def draw_board(self):
        # draw corners
        pygame.draw.rect(self.display, self.corner_color, pygame.Rect(0, 0, self.box_size, self.box_size))
        pygame.draw.rect(self.display, self.corner_color, pygame.Rect(0, self.height-self.box_size, self.box_size, self.box_size))
        pygame.draw.rect(self.display, self.corner_color, pygame.Rect(self.width-self.box_size, 0, self.box_size, self.box_size))
        pygame.draw.rect(self.display, self.corner_color, pygame.Rect(self.width-self.box_size, self.height-self.box_size, self.box_size, self.box_size))
        # draw lines
        for x in range(self.num_boxes + 1):
            pygame.draw.line(self.display, self.line_color, (x*self.box_size, 0), (x*self.box_size, self.height), 2)
        for y in range(self.num_boxes + 1):
            pygame.draw.line(self.display, self.line_color, (0, y*self.box_size), (self.width, y*self.box_size), 2)

    def draw_pieces(self):
        # parse position
        position = self.position.split(',')

        # draw king
        king = len(bin(int(position[0])).split('1', 1)[1])
        king_x = king // 11
        king_y = king - king_x * 11
        self.display.blit(self.images[0], (king_x * self.box_size, king_y * self.box_size))

        # draw white pieces
        white_pieces = bin(int(position[1]))[2:]
        while int(white_pieces) > 0:
            piece = len(white_pieces.split('1', 1)[1])
            piece_x = piece // 11
            piece_y = piece - piece_x * 11
            self.display.blit(self.images[1], (piece_x * self.box_size, piece_y * self.box_size))
            white_pieces = white_pieces.split('1', 1)[1]
        
        # draw black pieces
        black_pieces = bin(int(position[2]))[2:]
        while int(black_pieces) > 0:
            piece = len(black_pieces.split('1', 1)[1])
            piece_x = piece // 11
            piece_y = piece - piece_x * 11
            self.display.blit(self.images[2], (piece_x * self.box_size, piece_y * self.box_size))
            black_pieces = black_pieces.split('1', 1)[1]

    def draw(self):
        self.is_running = True
        while self.is_running:
            self.handle_events()
            self.display.fill(self.bg_color)
            self.draw_board()
            self.draw_pieces()
            pygame.display.flip()

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('position')
    args = parser.parse_args()
    bv = BitboardVisualizer(args.position)
    bv.draw()