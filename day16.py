#!/usr/bin/env pipenv run python

import re

from get_input import get_input

def make_func(func):
    def wrapped(reg, val):
        val = list(val)
        val[reg[3]] = func(reg, val)
        return tuple(val)
    return wrapped 

funcs = {
    'addr': make_func(lambda reg, val: val[reg[1]] + val[reg[2]]),
    'addi': make_func(lambda reg, val: val[reg[1]] + reg[2]),
    'mulr': make_func(lambda reg, val: val[reg[1]] * val[reg[2]]),
    'muli': make_func(lambda reg, val: val[reg[1]] * reg[2]),
    'banr': make_func(lambda reg, val: val[reg[1]] & val[reg[2]]),
    'bani': make_func(lambda reg, val: val[reg[1]] & reg[2]),
    'borr': make_func(lambda reg, val: val[reg[1]] | val[reg[2]]),
    'bori': make_func(lambda reg, val: val[reg[1]] | reg[2]),
    'setr': make_func(lambda reg, val: val[reg[1]]),
    'seti': make_func(lambda reg, val: reg[1]),
    'gtir': make_func(lambda reg, val: 1 if reg[1] > val[reg[2]] else 0),
    'gtri': make_func(lambda reg, val: 1 if val[reg[1]] > reg[2] else 0),
    'gtrr': make_func(lambda reg, val: 1 if val[reg[1]] > val[reg[2]] else 0),
    'eqir': make_func(lambda reg, val: 1 if reg[1] == val[reg[2]] else 0),
    'eqri': make_func(lambda reg, val: 1 if val[reg[1]] == reg[2] else 0),
    'eqrr': make_func(lambda reg, val: 1 if val[reg[1]] == val[reg[2]] else 0),
}

def part1(tests):
    """Solution to part 1"""
    count = 0
    for before, registers, after in tests:
        works = 0
        for func in funcs.values():
            if after == func(registers, before):
                works += 1
        count += 1 if works >= 3 else 0
    return count

def part2(tests, program):
    """Solution to part 2"""
    possible_optcodes = {n: set(funcs.keys()) for n in range(16)}
    for before, registers, after in tests:
        for name, func in funcs.items():
            if after != func(registers, before):
                possible_optcodes[registers[0]].discard(name)
    optcodes = {}
    while any(len(v) == 1 for v in possible_optcodes.values()):
        code, name = next((k, v) for k, v in possible_optcodes.items() if len(v) == 1)
        name = name.pop()
        optcodes[code] = name
        del possible_optcodes[code]
        for value in possible_optcodes.values():
            value.discard(name)
    assert possible_optcodes == {}
    values = (0,) * 4
    for step in program:
        values = funcs[optcodes[step[0]]](step, values)
    return values[0]

def parse(text):
    groups = (
        re.compile(r'Before:\s+\[(\d+), (\d+), (\d+), (\d+)\]'),
        re.compile(r'(\d+) (\d+) (\d+) (\d+)'),
        re.compile(r'After:\s+\[(\d+), (\d+), (\d+), (\d+)\]'),
        )
    tests = []
    blank_line = None 
    lines = iter(text.splitlines())
    for i, line in enumerate(lines): 
        if line == '':
            if blank_line:
                break
            blank_line = True
            continue

        blank_line = False 
        if i % 4 == 0:
            tests.append([])
        match = groups[i % 4].match(line)
        tests[-1].append(tuple(int(n) for n in match.groups()))

    next(lines) # Skip one blank line
    events = []
    for line in lines:
        match = groups[1].match(line)
        events.append(tuple(int(n) for n in match.groups()))

    return tuple(tuple(t) for t in tests), tuple(events)


if __name__ == '__main__':
    tests, events = parse(get_input(day=16, year=2018))
    print("Part 1: {}".format(part1(tests)))
    print("Part 2: {}".format(part2(tests, events)))
