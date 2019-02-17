#!/usr/bin/env pipenv run python

import collections
import re

from get_input import get_input

class Unit:
    def __init__(self, hp, attributes, ap, damage_type, initiative):
        self.hp = int(hp)
        self.ap = int(ap)
        self.damage_type = damage_type
        self.initiative = int(initiative)
        self.weak = []
        self.immune = []
        if attributes:
            for group in attributes.split('; '):
                if group.startswith('weak to '):
                    self.weak = group[8:].split(',')
                elif group.startswith('immune to '):
                    self.immune = group[10:].split(',')
                else:
                    raise ValueError(f"What is this? {attributes}")

    def __repr__(self):
        hp = f"hp={self.hp}"
        ap = f"ap={self.ap}"
        damage_type = f"damage_type={self.damage_type}"
        initiative = f"initiative={self.initiative}"
        attributes = f"'weak to {','.join(self.weak)}; immune to {','.join(self.immune)})"
        return f"Unit({hp}, {ap}, {damage_type}, {initiative}, {attributes})"

def part1(immune, infection):
    """Solution to part 1"""
    pass

def part2(immune, infection):
    """Solution to part 2"""
    pass

def parse(text):
    immune = []
    infection = []

    unit = re.compile(
        r'(?P<units>\d+) units each with (?P<hp>\d+) hit points '
        r'(:?\((?P<attributes>.*?)\) )?'
        r'with an attack that does (?P<ap>\d+) (?P<damage_type>\w+) damage '
        r'at initiative (?P<initiative>\d+)'
    )
    for line in text.splitlines():
        if line == '':
            continue
        elif line == "Immune System:":
            army = immune
        elif line == "Infection:":
            army = infection
        else:
            m = unit.match(line)
            groups = m.groupdict()
            units = groups['units']
            del groups['units']
            for _ in range(int(units)):
                army.append(Unit(**groups))
    return immune, infection

if __name__ == '__main__':
    armies = parse(get_input(day=24, year=2018))
    print("Part 1: {}".format(part1(*armies)))
    print("Part 2: {}".format(part2(*armies)))
    import pdb; pdb.set_trace()
