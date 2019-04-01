#!/usr/bin/env pipenv run python

import collections
import copy

from get_input import get_input

class Point(collections.namedtuple("Point", ["x", "y"])):
    def __repr__(self):
        return f"({self.x}, {self.y})"

    def __add__(self, other):
        return Point(self.x + other.x, self.y + other.y)

    def __hash__(self):
        return hash((self.x, self.y))


class Cart:
    _DIRECTIONS = {
        'v': Point(0, 1),
        '>': Point(1, 0),
        '<': Point(-1, 0),
        '^': Point(0, -1)
        }

    _TURNS = {
        ('v', '/'): '<', ('v', '\\'): '>',
        ('>', '/'): '^', ('>', '\\'): 'v',
        ('^', '/'): '>', ('^', '\\'): '<',
        ('<', '/'): 'v', ('<', '\\'): '^',
    }

    _TURN_MAP = ('>', 'v', '<', '^')

    def __init__(self, direction, position):
        self.direction = direction
        self.position = position
        self._turns = 0

    def __repr__(self):
        return f"Cart({self.direction}, {self.position})"

    def __str__(self):
        return self.direction

    def move(self):
        self.position += self._DIRECTIONS[self.direction]

    def turn(self, segment):
        if segment == '+':
            self._turns %= 3
            current = self._TURN_MAP.index(self.direction)
            if self._turns == 0:
                self.direction = self._TURN_MAP[(current - 1)]
            elif self._turns == 2:
                self.direction = self._TURN_MAP[(current + 1) % len(self._TURN_MAP)]
            self._turns += 1
        else:
            self.direction = self._TURNS.get(
                (self.direction, segment),
                self.direction)

    def collision(self, other):
        return self.position == other.position and self != other

    def __lt__(self, other):
        return (self.position.x, self.position.y) < (other.position.x, other.position.y)


def print_track(track, carts):
    track = [list(row) for row in track]
    for cart in carts:
        track[cart.position.y][cart.position.x] = cart.direction
    print(''.join('\n' + ''.join(row) for row in track))


def part1(track, carts):
    """Solution to part 1"""
    carts = copy.deepcopy(carts)
    while True:
        carts.sort()
        for cart in carts:
            cart.move()
            segment = track[cart.position.y][cart.position.x]
            cart.turn(segment)
            if any(c.collision(cart) for c in carts):
                return f"{cart.position.x},{cart.position.y}"

def part2(track, carts):
    """Solution to part 2"""
    # output = open('output_13.failing', 'w')
    carts = set(copy.deepcopy(carts))
    while len(carts) > 1:
        for cart in sorted(carts):
            cart.move()
            segment = track[cart.position.y][cart.position.x]
            cart.turn(segment)
            for c in list(carts):
                if c.collision(cart):
                    carts.remove(c)
                    carts.remove(cart)
                    break
    pos = list(carts)[0].position
    return f"{pos.x},{pos.y}"

Record = collections.namedtuple('Record', 'input output')

def make_track(lines):
    lines = tuple(lines.splitlines())
    cart_types = {'v': '|', '>': '-', '^': '|', '<': '-'}
    track = []
    carts = []
    for y, row in enumerate(lines):
        track.append([])
        for x, char in enumerate(row):
            if char in cart_types:
                carts.append(Cart(char, Point(x, y)))
                char = cart_types[char]
            assert char in ' |\\/+-'
            track[-1].append(char)
    assert all(len(row) == len(track[0]) for row in track)
    return tuple(tuple(t) for t in track), carts

TEST_TRACK_1 = """/->-\        
|   |  /----\\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   """

TEST_TRACK_2 = """/>-<\  
|   |  
| /<+-\\
| | | v
\>+</ |
  |   ^
  \<->/"""

if __name__ == '__main__':
    assert '7,3' == part1(*make_track(TEST_TRACK_1))
    TRACK, CARTS = make_track(get_input(day=13, year=2018))
    print("Part 1: {}".format(part1(TRACK, CARTS)))
    # NOT (58,119), (
    # assert '6,4' == part2(*make_track(TEST_TRACK_2))
    print("Part 2: {}".format(part2(TRACK, CARTS)))
