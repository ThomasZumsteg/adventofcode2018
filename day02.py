#!/usr/bin/env pipenv run python
"""Solutions to day 2 of Advent of Code"""

from collections import defaultdict, Counter
from get_input import get_input

def part1(lines):
    """Solution to part 1"""
    checksum = defaultdict(int)
    for line in lines:
        digits = Counter(list(line))
        for i in range(2, 5):
            if i in digits.values():
                checksum[i] += 1
    result = 1
    for num in checksum.values():
        result *= num
    return result


def part2(lines):
    """Solution to part 2"""
    for h_index, head in enumerate(lines):
        for tail in lines[:h_index]:
            common_letters = ''.join(
                h for h, t in zip(list(head), list(tail)) if h == t)
            if len(common_letters) + 1 == len(head):
                return common_letters
    raise Exception("Could not find soltution")

if __name__ == '__main__':
    LINES = get_input(day=2, year=2018).splitlines()
    print("Part 1: {}".format(part1(LINES)))
    print("Part 2: {}".format(part2(LINES)))
