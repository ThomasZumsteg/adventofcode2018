#!/usr/bin/env pipenv run python

import collections
import re
import enum
import heapq

from get_input import get_input

class Point(collections.namedtuple("Point", ["x", "y"])):
    def __repr__(self):
        return f"({self.x}, {self.y})"

    def __add__(self, other):
        if not isinstance(other, Point):
            return NotImplemented
        return type(self)(self.x + other.x, self.y + other.y)

    @classmethod
    def parse(cls, line):
        m = re.match(r'target: (\d+),(\d+)', line)
        return cls(*(int(g) for g in m.groups()))

    def distance(self, other):
        return abs(self.x - other.x) + abs(self.y - other.y)

Point.DIRECTIONS = (
    Point(0, 1),
    Point(0, -1),
    Point(1, 0),
    Point(-1, 0),
)


class GeologicalMap(dict):
    TYPES = {
        0: '.',
        1: '=',
        2: '|'
    }

    def __init__(self, depth, target, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.depth = depth
        self.target = target

    def __getitem__(self, point):
        if point not in self:
            raise ValueError("Point needs to be greater than Point(0, 0)")
        if not super().__contains__(point):
            if point.x == 0 or point.y == 0:
                geological_index = point.x * 16807 + point.y * 48271
            elif point == self.target:
                geological_index = 0
            else:
                geological_index = self[point+Point(-1, 0)] *\
                                   self[point+Point(0, -1)]
            super().__setitem__(point, (geological_index + self.depth) % 20183)
        return super().__getitem__(point)

    def __str__(self):
        result = []
        for y in range(self.target.y + 1):
            result.append([])
            for x in range(self.target.x + 1):
                point = Point(x, y)
                if point == self.target:
                    char = 'T'
                elif point == Point(0, 0):
                    char = 'M'
                else:
                    char = GeologicalMap.TYPES[self[point] % 3]
                result[-1].append(char)
        return '\n'.join(''.join(row) for row in result)

    def __contains__(self, point):
        return point.x >= 0 and point.y >= 0

def part1(depth, target):
    """Solution to part 1"""
    mapping = GeologicalMap(depth, target)
    total = 0
    for y in range(target.y+1):
        for x in range(target.x+1):
            total += mapping[Point(x, y)] % 3
    return total

def part2(depth, target):
    """Solution to part 2"""
    class Gear(enum.Enum):
        NONE = 0 # WET (1), NARROW (2)
        TORCH = 1 # ROCKY (0), NARROW (2)
        CLIMB = 2 # ROCKY (0), WET (1)

        def __lt__(self, other):
            return self.value < other.value # pylint: disable=comparison-with-callable

        def get_compliment(self, terrain):
            return Gear(3 - terrain - self.value)


    mapping = GeologicalMap(depth, target)
    queue = [(Point(0, 0).distance(target), Point(0, 0), Gear.TORCH)]
    seen = set()
    path_map = {}

    while queue:
        _, position, gear = heapq.heappop(queue)
        print(f"{len(queue):5d} - {position}")
        if position == mapping.target and gear == Gear.TORCH:
            import pdb; pdb.set_trace()
        if (position, gear) in seen:
            continue
        seen.add((position, gear))

        other_gear = gear.get_compliment(mapping[position] % 3)
        path_map.setdefault((position, other_gear), (position, gear))
        heapq.heappush(queue, (target.distance(position), position, other_gear))

        for direction in Point.DIRECTIONS:
            new_position = position + direction
            heapq.heappush(
                queue, (target.distance(new_position), new_position, gear))
            path_map.setdefault((new_position, gear), (position, gear))

    raise ValueError("Cannot be done")


def parse(text):
    lines = text.splitlines()
    depth = int(lines[0].strip().split(' ')[1])
    target = Point.parse(lines[1])
    return depth, target

if __name__ == '__main__':
    assert part1(510, Point(10, 10)) == 114
    puzzle_args = parse(get_input(day=22, year=2018).strip())
    print("Part 1: {}".format(part1(*puzzle_args)))
    assert part2(510, Point(10, 10)) == 45
    print("Part 2: {}".format(part2(*puzzle_args)))
