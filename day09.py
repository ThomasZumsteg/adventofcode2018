#!/usr/bin/env pipenv run python
"""Solutions to day 9 of Advent of Code"""

import re
import itertools
import collections

from get_input import get_input, line_parser

class Marble:
    __slots__ = ['value', 'head', 'tail']

    def __init__(self, value, tail=None, head=None):
        self.value = value
        self.tail = tail or self
        self.head = head or self

    def insert(self, marble):
        other = Marble(marble, tail=self, head=self.head)
        self.head, other.head.tail = other, other
        return other

    def remove(self):
        self.head.tail, self.tail.head = self.tail, self.head
        return self.value, self.head

    def move(self, n=1):
        result = self
        for _ in range(abs(n)):
            result = result.head if n >= 0 else result.tail
        return result

    def __repr__(self):
        return f"Marble({self.value}, tail=Marble({self.tail.value}), "\
               f"head=Marble({self.head.value}))"

def part1(game):
    """Solution to part 1"""
    root = Marble(0)
    scores = {e: 0 for e in range(1, game[0].players+1)}
    for elf, marble in zip(itertools.cycle(
            range(1, game[0].players+1)),
            range(1, game[0].marbles+1)):
        if marble % 23 == 0:
            val, root =  root.move(-7).remove()
            scores[elf] += marble + val
        else:
            root = root.move().insert(marble)
    return max(scores.values())

def part2(game):
    """Solution to part 2"""
    return part1([Record(game[0].players, game[0].marbles * 100)])

Record = collections.namedtuple('Record', 'players, marbles')

def parse(line):
    # \d+ players; last marble is worth \d+ points
    m = re.match(r"(\d+) players; last marble is worth (\d+) points", line)
    return Record(*[int(g) for g in m.groups()])

if __name__ == '__main__':
    assert part1([Record(9, 25)]) == 32
    assert part1([Record(13, 7999)]) == 146373
    assert part1([Record(17, 1104)]) == 2764
    assert part1([Record(21, 6111)]) == 54718
    assert part1([Record(30, 5807)]) == 37305
    LINES = line_parser(get_input(day=9, year=2018), parse=parse)
    print("Part 1: {}".format(part1(LINES)))
    print("Part 2: {}".format(part2(LINES)))
