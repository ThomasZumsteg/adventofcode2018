#!/usr/bin/env pipenv run python

import collections
import logging
import re

from get_input import get_input

Instruction = collections.namedtuple('Instruction', ('instr', 'reg'))

log = logging.getLogger(__name__)
log.setLevel(logging.WARNING)

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
        val = val.copy()
        val[reg[2]] = func(reg, val)
        return val
    return wrapped

def factors(num):
    for n in range(1, int(num ** 0.5) + 1):
        if num % n == 0:
            yield n
            yield num // n

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
        if values[ip] == 1:
            # SHORT CUT!
            break
        line = program[values[ip]]
        old_ip = values[ip]
        values = funcs[line.instr](line.reg, values)
        log.debug("%2d %4s %2d %2d %2d [%6d,%6d,%6d,%6d,%6d,%6d]",
                  old_ip, line.instr, *line.reg, *values)
        values[ip] += 1
    log.info('====================DONE====================')
    return sum(factors(values[1]))

def part2(*, ip, program):
    """Solution to part 2, based on static analysis"""
    values = [1, 0, 0, 0, 0, 0]
    while 0 <= values[ip] < len(program):
        if values[ip] == 1:
            # SHORT CUT!
            break
        line = program[values[ip]]
        old_ip = values[ip]
        values = funcs[line.instr](line.reg, values)
        log.debug("%2d %4s %2d %2d %2d [%6d,%6d,%6d,%6d,%6d,%6d]",
                  old_ip, line.instr, *line.reg, *values)
        values[ip] += 1
    log.info('====================DONE====================')
    return sum(factors(values[1]))

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
    args = parse(get_input(day=19, year=2018))
    print("Part 1: {}".format(part1(**args)))
    print("Part 2: {}".format(part2(**args)))


"""
p1_limit = 915
p2_limit = 10551315
total = 0
for i in range(limit):
    for j in range(limit):
        if i * j == limit:
            total += i
return total
"""

"""
#ip 2
0  addi 2 16 2 # goto 16
1  seti 1 1 3 # for r3 in range(r1)
    2  seti 1 7 5 # for r5 in range(r1)
        3  mulr 3 5 4
        4  eqrr 4 1 4 # if r3 * r5 == r1 add r3 to r0
            5  addr 4 2 2 # jump r4
            6  addi 2 1 2 # goto 7
            7  addr 3 0 0 # r3 + r0 => r0
    8  addi 5 1 5 # for r5 in range(1, r1):
    9  gtrr 5 1 4 # if r5 > r1 goto 2 else r3++
    10 addr 2 4 2 # jump r4
    11 seti 2 3 2 # goto 2
    12 addi 3 1 3 # r3++
13 gtrr 3 1 4     # if r3 > r1 EXIT else goto 1
    14 addr 4 2 2 # jump r4
    15 seti 1 9 2 # goto 1
    16 mulr 2 2 2 # EXIT
# sum(j for i in range(1, r1) for j in range(1, r1) if i * j == r1)
17 addi 1 2 1
18 mulr 1 1 1
19 mulr 2 1 1
20 muli 1 11 1
21 addi 4 3 4
22 mulr 4 2 4
23 addi 4 13 4
24 addr 1 4 1
25 addr 2 0 2 # jump r0
26 seti 0 1 2 # goto 0
27 setr 2 0 4
28 mulr 4 2 4
29 addr 2 4 4
30 mulr 2 4 4
31 muli 4 14 4
32 mulr 4 2 4
33 addr 1 4 1
34 seti 0 4 0
35 seti 0 5 2
"""
