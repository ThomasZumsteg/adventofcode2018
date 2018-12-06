#!/usr/bin/env pipenv run python
"""Solutions to day 5 of Advent of Code"""

from get_input import get_input

def reduce(polymer):
    """Reduce string as much as possible by removing matching letter pairs
    A matching pair is one uppercase letter and one lowercase
    example:
        abBa => aa
    """
    new_polymer = tuple(polymer)
    polymer = None
    while not new_polymer == polymer:
        polymer = tuple(new_polymer)
        new_polymer = ()
        for p in polymer:
            if new_polymer and new_polymer[-1] != p and new_polymer[-1].lower() == p.lower():
                new_polymer = new_polymer[:-1]
            else:
                new_polymer += (p,)
    return polymer


def part1(polymer):
    """Solution to part 1"""
    return len(reduce(polymer))

def part2(polymer):
    """Solution to part 2"""
    minimum = None
    for p in set(l.lower() for l in list(polymer)):
        processed = polymer.replace(p.upper(), '').replace(p, '')
        p_len = len(reduce(processed))
        if minimum is None or p_len < minimum:
            minimum = p_len
    return minimum


if __name__ == '__main__':
    LINE = get_input(day=5, year=2018).strip()
    print("Part 1: {}".format(part1(LINE)))
    print("Part 2: {}".format(part2(LINE)))
