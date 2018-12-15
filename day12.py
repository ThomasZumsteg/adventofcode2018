#!/usr/bin/env pipenv run python
"""Solutions to day 12 of Advent of Code"""

import re
import itertools
import collections

from get_input import get_input, line_parser

class DefaultList(list):
    def __init__(self, *args, default='.'):
        self._default = default
        self._negative = {}
        super().__init__(*args)

    def __getitem__(self, i):
        if isinstance(i, slice):
            return DefaultList(
                (self._get_with_default(n)
                for n in range(i.start or 0, i.stop, i.step or 1)),
                default=self._default)
        return self._get_with_default(i)

    def _get_with_default(self, i):
        if 0 < i <= len(self):
            return super().__getitem__(i)
        return self._default

def part1(records, iters=20):
    """Solution to part 1"""
    new = records[0]
    transform = {r.input: r.output for r in records[1:]}
    offset = 0
    for g in range(iters):
        old = new.copy()
        for i in range(len(old)-5):
            new[i] = transform.get(old[i:i+5], '.')
    return sum(i for i, n in enumerate(new, start=offset) if n == "#")

def part2(records):
    """Solution to part 2"""
    return part1(records, iters=50000000000)

Record = collections.namedtuple('Record', 'input output')

def parse(line):
    m = re.match(r"initial state: ([.#]+)", line)
    if m:
        return m.group(1)
    m = re.match(r"([.#]{5}) => ([.#])", line)
    return Record(m.group(1), m.group(2))

if __name__ == '__main__':
    LINES = line_parser(get_input(day=12, year=2018), parse=parse)
    print("Part 1: {}".format(part1(LINES)))
    print("Part 2: {}".format(part2(LINES)))
