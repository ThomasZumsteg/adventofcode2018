#!/usr/bin/env pipenv run python

import collections
import logging
import re

from get_input import get_input

Instruction = collections.namedtuple('Instruction', ('instr', 'reg'))

log = logging.getLogger(__name__)
log.setLevel(logging.DEBUG)

fh = logging.FileHandler('day19.log', mode='w+')
fh.setFormatter(logging.Formatter('%(message)s'))
fh.setLevel(logging.DEBUG)
log.addHandler(fh)

ch = logging.StreamHandler()
ch.setFormatter(logging.Formatter('%(message)s'))
ch.setLevel(logging.INFO)
log.addHandler(ch)

def make_func(func):
    def wrapped(reg, val):
        val = list(val)
        val[reg[2]] = func(reg, val)
        return tuple(val)
    return wrapped

funcs = {
    'addr': make_func(lambda reg, val: val[reg[0]] + val[reg[1]]),
    'addi': make_func(lambda reg, val: val[reg[0]] + reg[1]),
    'mulr': make_func(lambda reg, val: val[reg[0]] * val[reg[1]]),
    'muli': make_func(lambda reg, val: val[reg[0]] * reg[1]),
    'banr': make_func(lambda reg, val: val[reg[0]] & val[reg[1]]),
    'bani': make_func(lambda reg, val: val[reg[0]] & reg[1]),
    'borr': make_func(lambda reg, val: val[reg[0]] | val[reg[1]]),
    'bori': make_func(lambda reg, val: val[reg[0]] | reg[1]),
    'setr': make_func(lambda reg, val: val[reg[0]]),
    'seti': make_func(lambda reg, val: reg[0]),
    'gtir': make_func(lambda reg, val: 1 if reg[0] > val[reg[1]] else 0),
    'gtri': make_func(lambda reg, val: 1 if val[reg[0]] > reg[1] else 0),
    'gtrr': make_func(lambda reg, val: 1 if val[reg[0]] > val[reg[1]] else 0),
    'eqir': make_func(lambda reg, val: 1 if reg[0] == val[reg[1]] else 0),
    'eqri': make_func(lambda reg, val: 1 if val[reg[0]] == reg[1] else 0),
    'eqrr': make_func(lambda reg, val: 1 if val[reg[0]] == val[reg[1]] else 0),
}

def part1(*, ip, program):
    """Solution to part 1"""
    values = [0, 0, 0, 0, 0, 0]
    while 0 <= values[ip] < len(program):
        line = program[values[ip]]
        new = funcs[line.instr](line.reg, values)
        log.debug(
            "[%6d,%6d,%6d,%6d,%6d,%6d] %4s %2d %2d %2d [%6d,%6d,%6d,%6d,%6d,%6d]",
            *values, line.instr, *line.reg, *new)
        values = list(new)
        values[ip] += 1
    return values[0]

def part2(*, ip, program):
    """Solution to part 2"""
    values = [1, 0, 0, 0, 0, 0]
    while 0 <= values[ip] < len(program):
        line = program[values[ip]]
        values = funcs[line.instr](line.reg, values)
        values = list(values)
        values[ip] += 1
    return values[0]

def parse(text):
    lines = iter(text.splitlines())
    ip = int(next(lines).split(' ')[1])
    program = []
    program_re = re.compile(r'(\w{4}) (\d+) (\d+) (\d+)')
    for line in lines:
        m = program_re.match(line).groups()
        program.append(Instruction(m[0], tuple(int(n) for n in m[1:])))
    return {'ip': ip, 'program': tuple(program)}

test = """#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"""

if __name__ == '__main__':
    args = parse(test)
    part1(**args)
    args = parse(get_input(day=19, year=2018))
    print("Part 1: {}".format(part1(**args)))
    print("Part 2: {}".format(part2(**args)))
