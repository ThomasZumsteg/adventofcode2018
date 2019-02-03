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
Point.RIGHT, Point.LEFT = Point(1, 0), Point(-1, 0)


class Mapping(dict):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.reverse_mapping = collections.defaultdict(set)
        self.water_front = set()
        self._min, self._max = None, None

    @classmethod
    def make_mapping(cls, rows):
        mapping = cls()
        for coord_i, val_i, coord_j, start_j, end_j in rows:
            for val_j in range(start_j, end_j+1):
                mapping[Point(**{coord_i: val_i, coord_j: val_j})] = '#'
        return mapping

    @property
    def max(self):
        if self._max is None:
            self._max = Point(
                max((p for p, v in self.items() if v == '#'), key=lambda p: p.x).x,
                max((p for p, v in self.items() if v == '#'), key=lambda p: p.y).y)
            self._max += Point(1, 0)
        return self._max

    @property
    def min(self):
        if self._min is None:
            self._min = Point(
                min((p for p, v in self.items() if v == '#'), key=lambda p: p.x).x,
                min((p for p, v in self.items() if v == '#'), key=lambda p: p.y).y)
            self._min += Point(-1, 0)
        return self._min

    def __str__(self):
        lines = []
        smallest, largest = self.min, self.max
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
        try:
            self.reverse_mapping[self[key]].remove(key)
        except KeyError:
            pass
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
        front, self.water_front = self.water_front, set()
        for space in front:
            assert isinstance(space, Point)
            if space.y >= self.max.y:
                continue
            if self[space] == '+':
                if self[space + Point.DOWN] == '.':
                    self[space + Point.DOWN] = '|'
                    self.water_front.add(space + Point.DOWN)
                else:
                    raise NotImplementedError("No empty space below source")
            elif self[space] == '|':
                if self[space + Point.DOWN] == '.':
                    self[space + Point.DOWN] = '|'
                    self.water_front.add(space + Point.DOWN)
                elif self[space + Point.DOWN] in ('#', '~'):
                    char, row, boundary = self.fill_row(space)
                    for item in row:
                        self[item] = char
                    assert boundary != set()
                    self.water_front.update(boundary)
                elif self[space + Point.DOWN] == '|':
                    pass

    def fill_row(self, space):
        queue = [(Point.LEFT, space), (Point.RIGHT, space)]
        row = set()
        boundery = set()
        bounded = True
        tracebacks = set()

        while queue:
            direction, point = queue.pop(0)
            if self[point] == '#':
                continue
            if self[point + Point.UP] == '|':
                tracebacks.add(point + Point.UP)
            row.add(point)
            if self[point + Point.DOWN] in ('.', '|'):
                if self[point + Point.DOWN] == '.':
                    boundery.add(point)
                bounded = False
            elif self[point + Point.DOWN] in ('#', '~'):
                queue.append((direction, point + direction))
            else:
                import pdb; pdb.set_trace()
        char, return_val = '|', tuple(boundery)
        if bounded:
            char = '~'
            return_val = tracebacks
        return char, row, return_val

def part1(clay):
    """Solution to part 1"""
    water_map = Mapping.make_mapping(clay)
    water_map[Point(500, 0)] = '+'
    water_map.water_front = set((Point(500, 0),))
    while water_map.water_front != set():
        water_map.take_step()
    wet_spaces = 0
    for value in ('~', '|'):
        wet_spaces += sum(
            1 for p in water_map[value] if water_map.min.y <= p.y <= water_map.max.y)
    return wet_spaces

def part2(clay):
    """Solution to part 2"""
    water_map = Mapping.make_mapping(clay)
    water_map[Point(500, 0)] = '+'
    water_map.water_front = set((Point(500, 0),))
    while water_map.water_front != set():
        water_map.take_step()
    return sum(1 for p in water_map['~'] if water_map.min.y <= p.y <= water_map.max.y)


def parse(line):
    match = re.match(r"([xy])=(\d+), ([xy])=(\d+)..(\d+)", line)
    return tuple(int(n) if n.isdigit() else n for n in match.groups())

test = """x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"""

if __name__ == '__main__':
    test = line_parser(test, parse=parse)
    assert part1(test) == 57
    assert part2(test) == 29
    clay_map = line_parser(get_input(day=17, year=2018), parse=parse)
    print("Part 1: {}".format(part1(clay_map)))
    print("Part 2: {}".format(part2(clay_map)))
