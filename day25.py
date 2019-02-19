#!/usr/bin/env pipenv run python

import collections

from get_input import get_input, line_parser

class Point(collections.namedtuple("Point", ["x", "y", "z", "t"])):
    def __repr__(self):
        return f"({self.x}, {self.y}, {self.z}, {self.t})"

    def __add__(self, other):
        if not isinstance(other, Point):
            return NotImplemented
        return type(self)(self.x + other.x, self.y + other.y, self.z + other.z, self.t + other.t)

    def distance(self, other):
        return abs(self.x - other.x) + abs(self.y - other.y) +\
                abs(self.z - other.z) + abs(self.t - other.t)


def part1(points):
    """Solution to part 1"""
    queue = list(set((p,)) for p in points)
    counter = len(queue)
    while counter >= 0:
        contellation = queue.pop(0)
        updates = []
        for match in queue:
            if any(p.distance(c) <= 3 for c in contellation for p in match):
                updates.append(match)
        if updates:
            counter = len(queue)
        else:
            counter -= 1
        for update in updates:
            queue.remove(update)
            contellation |= update
        queue.append(contellation)
    return len(queue)


def parse(line):
    return Point(*(int(n) for n in line.split(',')))


TEST1 = """0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0"""

TEST2 = """-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0"""

if __name__ == '__main__':
    assert part1((parse(line) for line in TEST1.splitlines())) == 2
    assert part1((parse(line) for line in TEST2.splitlines())) == 4
    POINTS = line_parser(get_input(day=25, year=2018), parse=parse)
    print("Part 1: {}".format(part1(POINTS)))
