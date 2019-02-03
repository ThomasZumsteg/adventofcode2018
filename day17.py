#!/usr/bin/env pipenv run python

import collections
import re

from get_input import get_input, line_parser

class Point(collections.namedtuple("Point", ["x", "y"])):
    def __repr__(self):
        return f"({self.x}, {self.y})"

    def __add__(self, other):
        if not isinstance(other, Point):
            return NotImplemented
        return type(self)(self.x + other.x, self.y + other.y)

Point.UP, Point.DOWN = Point(0, -1), Point(0, 1)
Point.LEFT, Point.RIGHT = Point(1, 0), Point(-1, 0)


class Mapping(dict):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.reverse_mapping = collections.defaultdict(set)
        self.water_front = set()

    @classmethod
    def make_mapping(cls, rows):
        mapping = cls()
        for coord_i, val_i, coord_j, start_j, end_j in rows:
            for val_j in range(start_j, end_j+1):
                mapping[Point(**{coord_i: val_i, coord_j: val_j})] = '#'
        return mapping

    def max(self):
        return Point(
            max((p for p in self.keys()), key=lambda p: p.x).x,
            max((p for p in self.keys()), key=lambda p: p.y).y)

    def min(self):
        return Point(
            min((p for p in self.keys()), key=lambda p: p.x).x,
            min((p for p in self.keys()), key=lambda p: p.y).y)

    def __str__(self):
        lines = []
        smallest, largest = self.min(), self.max()
        for y in range(smallest.y, largest.y+1):
            lines.append("")
            for x in range(smallest.x, largest.x+1):
                lines[-1] += self.get(Point(x, y), '.')
        return '\n'.join(lines)

    def __getitem__(self, key):
        if isinstance(key, Point):
            return self.get(key, '.')
        return self.reverse_mapping[key]

    def __setitem__(self, key, value):
        if not isinstance(key, Point):
            raise ValueError(f"Key must be a Point, not {key}")
        super().__setitem__(key, value)
        self.reverse_mapping[value].add(key)

    def __delitem__(self, key):
        val = self[key]
        self.reverse_mapping[val].discard(key)
        if self.reverse_mapping[val] == {}:
            del self.reverse_mapping[val]
        super().__delitem__(key)

    def copy(self):
        new = type(self)()
        for k, v in self.items():
            new[k] = v
        return new

    def take_step(self):
        if self.water_front == set():
            self.water_front.update(self['+'])
        new_self = self.copy()
        new_self.water_front = set()
        for space in self.water_front:
            assert isinstance(space, Point)
            if not self.in_bounds(space):
                continue

            if self[space] == '+':
                if self[space + Point.DOWN] == '.':
                    new_self[space + Point.DOWN] = '|'
                    new_self.water_front.add(space + Point.DOWN)
                else:
                    raise NotImplementedError("No empty space below source")
            elif self[space] == '|':
                if self[space + Point.DOWN] == '.':
                    new_self[space + Point.DOWN] = '|'
                    new_self.water_front.add(space + Point.DOWN)
                else:
                    boundary = new_self.fill_row(space)
                    assert boundary != set()
                    new_self.water_front.update(boundary)
            else:
                raise NotImplementedError("What do now?")
        return new_self

    def in_bounds(self, other):
        lower_left = self.min()
        upper_right = self.max()
        return lower_left.x <= other.x and lower_left.y <= other.y and \
                upper_right.x >= other.x and upper_right.y >= other.y

    def fill_row(self, space):
        queue = [(Point.LEFT, space), (Point.RIGHT, space)]
        row = set()
        boundery = set()
        tracebacks = set()

        while queue:
            direction, point = queue.pop(0)
            if self[point] == '#':
                continue
            if self[point + Point.UP] == '|':
                tracebacks.add(point + Point.UP)
            row.add(point)
            if self[point + Point.DOWN] == '.':
                boundery.add(point)
            else:
                queue.append((direction, point + direction))
        char, return_val = '|', tuple(boundery)
        if boundery == set():
            char = '~'
            return_val = tracebacks
        for item in row:
            self[item] = char
        assert return_val != set()
        return return_val

def part1(clay):
    """Solution to part 1"""
    water_map = Mapping.make_mapping(clay)
    water_map[Point(500, 0)] = '+'
    water_map.water_front = set((Point(500, 0),))
    while water_map.water_front != set():
        water_map = water_map.take_step()
    import pdb; pdb.set_trace()
    count = 0
    for value in ('~', '|'):
        count += len(water_map[value])
    return count

def part2(tests):
    """Solution to part 2"""
    pass


def parse(line):
    match = re.match(r"([xy])=(\d+), ([xy])=(\d+)..(\d+)", line)
    return tuple(int(n) if n.isdigit() else n for n in match.groups())

if __name__ == '__main__':
    clay_map = line_parser(get_input(day=17, year=2018), parse=parse)
    print("Part 1: {}".format(part1(clay_map)))
    print("Part 2: {}".format(part2(clay_map)))
