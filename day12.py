#!/usr/bin/env pipenv run python
"""Solutions to day 12 of Advent of Code"""

import re
import itertools
import collections

from get_input import get_input, line_parser

class DefaultList(collections.defaultdict):
    def __init__(self, values, default='.'):
        super().__init__(lambda: default, values)
        self._default = default

    def __getitem__(self, i):
        if isinstance(i, slice):
            return (
                self._get_with_default(n)
                for n in range(i.start or 0, i.stop, i.step or 1))
        return self._get_with_default(i)

    def __setitem__(self, key, value):
        if value != self._default:
            return super().__setitem__(key, value)
        return self._default

    def _get_with_default(self, i):
        if self.first <= i < self.last:
            return super().__getitem__(i)
        return self._default

    def __repr__(self):
        return ''.join(str(self[i]) for i in range(self.first, self.last))

    def copy(self):
        return type(self)(self.items(), default=self._default)

    @property
    def first(self):
        return min(self.keys())

    @property
    def last(self):
        return max(self.keys()) + 1

def part1(records, iters=20):
    """Solution to part 1"""
    new = DefaultList(enumerate(records[0]))
    transform = {r.input: r.output for r in records[1:]}
    for _ in range(iters):
        new, old = DefaultList(()), new
        for i in range(old.first-2, old.last+2):
            new[i] = transform.get(''.join(old[i-2:i+3]), '.')
    return sum(i for i, n in new.items() if n == "#")

def part2(records, iters=50000000000):
    """Solution to part 2"""
    new = DefaultList(enumerate(records[0]))
    transform = {r.input: r.output for r in records[1:]}
    count = None
    for count in itertools.count():
        new, old = DefaultList(()), new
        for i in range(old.first-2, old.last+2):
            new[i] = transform.get(''.join(old[i-2:i+3]), '.')
        if str(new) == str(old):
            break
    diff = iters - count - 1
    assert new.first - old.first == 1
    return sum(i + diff for i, n in new.items() if n == "#")

Record = collections.namedtuple('Record', 'input output')

def parse(line):
    m = re.match(r"initial state: ([.#]+)", line)
    if m:
        return m.group(1)
    m = re.match(r"([.#]{5}) => ([.#])", line)
    return Record(m.group(1), m.group(2))

TEST = """initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"""

if __name__ == '__main__':
    TEST_INPUT = line_parser(TEST, parse=parse)
    assert part1(TEST_INPUT) == 325
    LINES = line_parser(get_input(day=12, year=2018), parse=parse)
    print("Part 1: {}".format(part1(LINES)))
    assert part1(TEST_INPUT, iters=100) == part2(TEST_INPUT, iters=100)
    print("Part 2: {}".format(part2(LINES)))
    # NOT 1900000000422
