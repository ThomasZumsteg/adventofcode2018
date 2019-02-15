#!/usr/bin/env pipenv run python

import collections
import re

from get_input import get_input, line_parser

class Point(collections.namedtuple("Point", ["x", "y", "z"])):
    def __repr__(self):
        return f"({self.x}, {self.y}, {self.z})"

    def __add__(self, other):
        if not isinstance(other, Point):
            return NotImplemented
        return type(self)(self.x + other.x, self.y + other.y, self.z + other.z)

    def distance(self, other):
        return abs(self.x - other.x) + abs(self.y - other.y) + abs(self.z - other.z)

class Bot(collections.namedtuple('Bot', ('radius', 'position'))):
    def __new__(cls, x, y, z, r):
        return super().__new__(cls, int(r), Point(int(x), int(y), int(z)))

def bots_in_range(position, radius, bots):
    for bot in bots:
        if position.distance(bot.position) <= radius:
            yield bot

def part1(bots):
    """Solution to part 1"""
    big_bot = max(bots, key=lambda b: b.radius)
    return sum(1 for b in bots_in_range(big_bot.position, big_bot.radius, bots))

def part2(bots):
    """Solution to part 2"""
    pass


PART1_TEST = """pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1"""


PART2_TEST = """pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"""


def parse(line):
    return Bot(*re.match(r'pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)', line).groups())

if __name__ == '__main__':
    assert part1(line_parser(PART1_TEST, parse=parse)) == 7
    bot_list = line_parser(get_input(day=23, year=2018), parse=parse)
    print("Part 1: {}".format(part1(bot_list)))
    assert part2(line_parser(PART2_TEST, parse=parse)) == 36
    print("Part 2: {}".format(part2(bot_list)))
