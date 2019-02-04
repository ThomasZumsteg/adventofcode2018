#!/usr/bin/env pipenv run python

import collections

from get_input import get_input, line_parser

class Point(collections.namedtuple("Point", ["x", "y"])):
    def __repr__(self):
        return f"({self.x}, {self.y})"

    def __add__(self, other):
        if not isinstance(other, Point):
            return NotImplemented
        return type(self)(self.x + other.x, self.y + other.y)


class Mapping(tuple):
    def __new__(cls, flat_map):
        return tuple.__new__(Mapping, (tuple(row) for row in flat_map))

    def __getitem__(self, item):
        if not isinstance(item, Point):
            return NotImplemented
        if item.x < 0 or item.y < 0:
            raise IndexError("Negatives not allowed")
        return super().__getitem__(item.y).__getitem__(item.x)

    def adjacent(self, point, value):
        surrounding = (
            Point(-1, -1), Point(0, -1), Point(1, -1),
            Point(-1, 0), Point(1, 0),
            Point(-1, 1), Point(0, 1), Point(1, 1)
        )
        count = 0
        for diff in surrounding:
            try:
                if self[diff + point] == value:
                    count += 1
            except IndexError:
                pass
        return count

    def step(self):
        new_self = []
        for r, row in enumerate(self):
            new_self.append([])
            for c, value in enumerate(row):
                point = Point(c, r)
                if value == '.' and self.adjacent(point, '|') >= 3:
                    value = '|'
                elif value == '|' and self.adjacent(point, '#') >= 3:
                    value = '#'
                elif value == '#' and not (
                        self.adjacent(point, '#') >= 1 and self.adjacent(point, '|')):
                    value = '.'
                new_self[-1].append(value)
        return type(self)(tuple(tuple(row) for row in new_self))

    def count(self, char):
        return sum(row.count(char) for row in self)

    def __str__(self):
        return '\n'.join(''.join(row) for row in self)


def part1(initial_forest, turns=10):
    """Solution to part 1"""
    forest = Mapping(initial_forest)
    for _ in range(turns):
        forest = forest.step()
    return forest.count('|') * forest.count('#')

def part2(initial_forest, turns=1000000000):
    """Solution to part 2"""
    forest = Mapping(initial_forest)
    seen = {}
    turn = 0
    while forest not in seen and turn < turns:
        seen[forest] = (turn, forest.step())
        _, forest = seen[forest]
        turn += 1
    cycle_start = seen[forest][0]
    cycle_length = turn - cycle_start
    forward_steps = (turns - turn) % cycle_length
    for _ in range(forward_steps):
        _, forest = seen[forest]
    return forest.count('|') * forest.count('#')

test = """.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."""

if __name__ == '__main__':
    lines = tuple(tuple(row) for row in test.splitlines())
    assert part1(lines) == 1147
    lines = line_parser(get_input(day=18, year=2018), parse=tuple)
    print("Part 1: {}".format(part1(lines)))
    print("Part 2: {}".format(part2(lines)))
