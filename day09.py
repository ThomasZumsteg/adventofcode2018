#!/usr/bin/env pipenv run python
"""Solutions to day 8 of Advent of Code"""

import re
import itertools
import collections

from get_input import get_input, line_parser

class MarbleCircle:
    def __init__(self):
        self._index = 0
        self._marbles = [0]

    @property
    def index(self):
        self._index %= len(self._marbles)
        return self._index + 1

    def play(self, marble):
        if marble % 23 == 0:
            self._index -= 7
            return self._marbles.pop(self.index % len(self._marbles)) + marble
        self._index += 2
        self._marbles.insert(self.index, marble)
        return 0

    def __str__(self):
        return ''.join(f"{m:2} " if i != self.index else f"{m:2})" for i, m in enumerate(self._marbles))

def part1(game):
    """Solution to part 1"""
    game = game[0]
    circle = MarbleCircle()
    scores = {e: 0 for e in range(1, game.players+1)}
    for elf, marble in zip(itertools.cycle(range(1, game.players+1)), range(1, game.marbles+1)):
        scores[elf] += circle.play(marble)
    return max(scores.values())

def part2(game):
    """Solution to part 2"""
    game = Record(game[0].players, game[0].marbles * 100)
    circle = MarbleCircle()
    scores = {e: 0 for e in range(1, game.players+1)}
    for elf, marble in zip(itertools.cycle(range(1, game.players+1)), range(1, game.marbles+1)):
        scores[elf] += circle.play(marble)
    return max(scores.values())

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
    # print("Part 2: {}".format(part2(LINES)))
