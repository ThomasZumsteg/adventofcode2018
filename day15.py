#!/usr/bin/env pipenv run python
"""Solutions to day 15 of Advent of Code"""

import re
import itertools
import collections
from dataclasses import dataclass

from get_input import get_input, line_parser

class Unit:
    def __init__(self, x, y):
        self.hp = 200
        self.ap = 3

    def move(self, board):
        queue = [(self.x, self.y, None)]
        seen = set()
        while queue:
            x, y, first = queue.pop(0)
            for dx, dy in ((-1, 0), (0, -1), (1, 0), (0, 1)):
                i, j = x + dx, y + dy
                if not (0 <= j < len(board) and 0 <= i < len(board[j])):
                    continue

                first = first or (i, j)
                if board[i][j] == '.':
                    queue.append((i, j, first))
                elif board[i][j] != self.t and board[i][j] != '#':
                    return first

def part1(board, units):
    """Solution to part 1"""
    while True:
        for unit in units:
            targets = unit.find_targets()
            if targets == []:
                return sum(u.hp for u in units)
            points = set()
            for target in targets:
                points.update(target.adjecent_squares())

            



def part2(board, units):
    """Solution to part 2"""


Record = collections.namedtuple('Record', '')

def parse(lines):
    board = []
    units = []
    for y, row in enumerate(lines.splitlines()):
        board.append([])
        for x, char in enumerate(row):
            if char == 'E' or char == 'G':
                units.append(Unit(x, y, char))
                char = '.'
            board[-1].append(char)
    return board, units


if __name__ == '__main__':
    board, units = parse(get_input(day=14, year=2018).strip())
    print("Part 1: {}".format(part1(board, units)))
    print("Part 2: {}".format(part2(board, units)))
