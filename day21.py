#!/usr/bin/env pipenv run python

import collections
import logging
import re

from get_input import get_input

Instruction = collections.namedtuple('Instruction', ('instr', 'reg'))

log = logging.getLogger(__name__)
log.setLevel(logging.DEBUG)

fh = logging.FileHandler('day21.log', mode='w+')
fh.setFormatter(logging.Formatter('%(message)s'))
fh.setLevel(logging.DEBUG)
log.addHandler(fh)

out = logging.getLogger(__name__ + 'out')
out.setLevel(logging.INFO)
ch = logging.StreamHandler()
ch.setFormatter(logging.Formatter('%(message)s'))
ch.setLevel(logging.INFO)
out.addHandler(ch)

FORMAT = "{:2d} {:4s} ({:8d} {:8d} {:8d}) [{:5d} {:5d} {:5d} {:15d} {:5d} {:6d}]"

def make_func(func):
    def wrapped(reg, val):
        val = val.copy()
        val[reg[2]] = func(reg, val)
        return val
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
    values_list = [[i, 0, 0, 0, 0, 0] for i in range(1)]
    solution = None
    steps = 0
    while solution is None:
        new_list = []
        for v, values in enumerate(values_list):
            if not 0 <= values[ip] < len(program):
                solution = v
                break
            line = program[values[ip]]
            if v == 0:
                log.debug(FORMAT.format(values[ip], line.instr, *line.reg, *values))
            values = funcs[line.instr](line.reg, values)
            values[ip] += 1
            new_list.append(values)
        out.info(steps)
        values_list = new_list
        steps += 1
    return solution

def part2(*, ip, program):
    """Solution to part 2"""
    pass

def parse(text):
    lines = iter(text.splitlines())
    ip = int(next(lines).split(' ')[1])
    program = []
    program_re = re.compile(r'(\w{4}) (\d+) (\d+) (\d+)')
    for line in lines:
        m = program_re.match(line).groups()
        program.append(Instruction(m[0], tuple(int(n) for n in m[1:])))
    return {'ip': ip, 'program': tuple(program)}

if __name__ == '__main__':
    args = parse(get_input(day=21, year=2018))
    print("Part 1: {}".format(part1(**args)))
    print("Part 2: {}".format(part2(**args)))

"""
#ip 4
0  seti     123        0 3
1  bani       3      456 3
2  eqri       3       72 3 # GOTO 0 if r3 == 72
3  addr       3        4 4 # JMP 
4  seti       0        0 4 # GOTO 0
5  seti       0        5 3 
    6  bori       3    65536 5
    7  seti 5557974        2 3
        8  bani       5      255 2
        9  addr       3        2 3
        10 bani       3 16777215 3
        11 muli       3    65899 3
        12 bani       3 16777215 3
        13 gtir     256        5 2 # GOTO 27 if 256 > r5 else GOTO 17
        14 addr       2        4 4 # 
        15 addi       4        1 4 # GOTO 17
        16 seti      27        9 4 # GOTO 27
        17 seti       0        0 2
            18 addi       2        1 1
            19 muli       1      256 1
            20 gtrr       1        5 1 # (r2++; GOTO 17) if r1 > r5 else GOTO 24
            21 addr       1        4 4 #
            22 addi       4        1 4 # GOTO 24
            23 seti      25        4 4 # GOTO 25
            24 addi       2        1 2 # r2++
        25 seti      17        6 4 # GOTO 17
        26 setr       2        2 5 # r2 = r5
        27 seti       7        1 4 # GOTO 7
    28 eqrr       3        0 2 # EXIT if r3 == r0 GOTO 5 (r0 is set by me)
29 addr       2        4 4 # EXIT if 2 > 0
30 seti       5        7 4 # GOTO 5
"""
