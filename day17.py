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

    def __lt__(self, other):
        if not isinstance(other, Point):
            return NotImplemented
        return self.x < other.x and self.y < other.y

    def __gt__(self, other):
        if not isinstance(other, Point):
            return NotImplemented
        return not (self < other or self == other)


class Mapping(dict):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.reverse_mapping = collections.defaultdict(set)
        self._water_front = set()

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
        queue = set((*self['+'], *self['|'], *self['~']))
        for space in queue:
            if self[space + Point(0, -1)] == '.' and not (self.min() < space < self.max()):
                import pdb; pdb.set_trace()

def part1(clay):
    """Solution to part 1"""
    new_water_map = Mapping.make_mapping(clay)
    new_water_map[Point(500, 0)] = '+'
    old_water_map = None
    assert new_water_map[next(iter(new_water_map['+'])) + Point(0, 1)] == '.'
    while old_water_map != new_water_map:
        old_water_map = new_water_map
        new_water_map = old_water_map.take_step()
    count = 0
    for value in ('~', '|'):
        count += len(new_water_map[value])
    return count

def part2(tests):
    """Solution to part 2"""
    pass


def parse(line):
    match = re.match(r"([xy])=(\d+), ([xy])=(\d+)..(\d+)", line)
    return tuple(int(n) if n.isdigit() else n for n in match.groups() )

if __name__ == '__main__':
    clay = line_parser(get_input(day=17, year=2018), parse=parse)
    print("Part 1: {}".format(part1(clay)))
    print("Part 2: {}".format(part2(clay)))
