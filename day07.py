#!/usr/bin/env pipenv run python
"""Solutions to day 7 of Advent of Code"""

import re
import itertools
import collections

from get_input import get_input, line_parser

Record = collections.namedtuple('Record', 'before after')

def helper(records):
    result = {}
    for record in records:
        if record.before not in result:
            result[record.before] = set()
        if record.after not in result:
            result[record.after] = set()
        result[record.before].add(record.after)
    return result

def part1(records):
    """Solution to part 1"""
    solution = ""
    todo = helper(records)
    while todo:
        ready = list(k for k in todo.keys() if all(k not in v for v in todo.values()))
        ready.sort()
        solution += ready[0]
        del todo[ready[0]]
    return solution

def part2(records, n_workers=5, base=60):
    """Solution to part 2"""
    todo = helper(records)
    workers = {}
    time = 0
    while len(workers) != 0 or len(todo) != 0:
        t_diff = min((t for t, _ in workers.values()), default=0) + 1
        time += t_diff
        for jid in tuple(workers):
            workers[jid][0] -= t_diff 
            if workers[jid][0] < 0:
                del workers[jid]
        ready = list(
                k for k in todo.keys() if 
                all(k not in v for _, v in workers.values()) and
                all(k not in v for v in todo.values()))
        ready.sort()
        while len(workers) < n_workers and ready:
            item = ready.pop(0)
            workers[item] = [ord(item) - ord('A') + base, todo[item]]
            del todo[item]
    return time - 1

def parse(line):
    m = re.match(r"Step (.) must be finished before step (.) can begin.", line)
    return Record(*m.groups())

if __name__ == '__main__':
    LINES = line_parser(get_input(day=7, year=2018), parse=parse)
    print("Part 1: {}".format(part1(LINES)))
    print("Part 2: {}".format(part2(LINES)))
