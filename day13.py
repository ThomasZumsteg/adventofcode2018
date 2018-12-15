#!/usr/bin/env pipenv run python
"""Solutions to day 13 of Advent of Code"""

import re
import itertools
import collections
from dataclasses import dataclass

from get_input import get_input, line_parser

@dataclass
class Point:
    x: int
    y: int

    def __hash__(self):
        return hash((self.x, self.y))

    def __add__(self, other):
        return Point(self.x + other.x, self.y + other.y)

DIRECTIONS = {
    'v': Point( 0,  1),
    '>': Point( 1,  0),
    '<': Point(-1,  0),
    '^': Point( 0, -1)
    }

TURNS = {
    ('v', '/'): '<', ('v', '\\'): '>',
    ('>', '/'): '^', ('>', '\\'): 'v',
    ('^', '/'): '>', ('^', '\\'): '<',
    ('<', '/'): 'v', ('<', '\\'): '^',
}

def intersection(cart, n_turn):
    turn_map = ('>', 'v', '<', '^')
    n_turn %= 3
    current = turn_map.index(cart)
    r_val = cart
    if n_turn == 0:
        r_val = turn_map[(current - 1)]
    elif n_turn == 2:
        r_val = turn_map[(current + 1) % len(turn_map)]
    return r_val

def print_current(pos, track, cart=None):
    grid = [list((line[(pos.x-1 if pos.x > 0 else 0):pos.x+2]))
        for line in track[(pos.y-1 if pos.y > 0 else 0):pos.y+2]]
    if cart is not None:
        grid[1][1] = cart
    print(''.join('\n' + ''.join(row) for row in grid))


def part1(track, carts):
    """Solution to part 1"""
    while True:
        old_carts, carts = carts, {}
        for old_pos, (cart, turn) in old_carts.items():
            pos = old_pos + DIRECTIONS[cart]
            if pos in carts or (old_pos in carts and pos in old_carts):
                return f"{pos.x},{pos.y}" 
            track_segment = track[pos.y][pos.x]
            if track_segment == '+':
                cart = intersection(cart, turn % 3)
                turn += 1
            else:
                cart = TURNS.get((cart, track_segment), cart)
            carts[pos] = (cart, turn)

def part2(track, carts):
    """Solution to part 2"""
    iters = 0
    while len(carts) > 1:
        iters += 1
        old_carts, carts = carts, dict()
        for old_pos, cart, turn in old_carts:
            pos = old_pos + DIRECTIONS[cart]
            track_segment = track[pos.y][pos.x]
            if pos in carts:
                del carts[pos]
                continue
            elif pos in old_carts and 
            if track_segment == '+':
                cart = intersection(cart, turn)
                turn += 1
            else:
                cart = TURNS.get((cart, track_segment), cart)
            carts[pos] = (cart, turn)
    last = list(carts)[0]
    print_current(last, track)
    return f"{last.x},{last.y}"

Record = collections.namedtuple('Record', 'input output')

def make_track(lines):
    lines = tuple(lines.splitlines())
    cart_types = { 'v': '|', '>': '-', '^': '|', '<': '-' }
    track = []
    carts = {}
    for y, row in enumerate(lines):
        track.append([])
        for x, char in enumerate(row):
            if char in cart_types:
                carts[Point(x, y)] = (char, 0)
                char = cart_types[char]
            assert char in ' |\\/+-'
            track[-1].append(char)
    assert all(len(row) == len(track[0]) for row in track)
    return tuple(tuple(t) for t in track), carts

test_track = """/>-<\  
|   |  
| /<+-\\
| | | v
\>+</ |
  |   ^
  \<->/"""

if __name__ == '__main__':
    track, carts = make_track(get_input(day=13, year=2018))
    print("Part 1: {}".format(part1(track, carts)))
    # track, carts = make_track(test_track)
    print("Part 2: {}".format(part2(track, carts)))
