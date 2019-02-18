#!/usr/bin/env pipenv run python

import collections
import re

from get_input import get_input

class Group:
    def __init__(self, units, hp, attributes, ap, damage_type, initiative):
        self.hp = int(hp)
        self.ap = int(ap)
        self.damage_type = damage_type
        self.initiative = int(initiative)
        self.units = int(units)
        self.weak = []
        self.immune = []
        if attributes:
            for group in attributes.split('; '):
                if group.startswith('weak to '):
                    self.weak = group[8:].split(', ')
                elif group.startswith('immune to '):
                    self.immune = group[10:].split(', ')
                else:
                    raise ValueError(f"What is this? {attributes}")

    def __repr__(self):
        unts = f"units={self.units}"
        hp = f"hp={self.hp}"
        ap = f"ap={self.ap}"
        damage_type = f"damage_type={self.damage_type}"
        initiative = f"initiative={self.initiative}"
        attributes = f"'weak to {','.join(self.weak or ('None',))}; immune to {','.join(self.immune or ('None',))}"
        return f"{self.__class__.__name__}({unts}, {hp}, {ap}, {damage_type}, {initiative}, {attributes})"

    @property
    def effective_power(self):
        return self.ap * self.units

    def damage(self, other):
        bonus = 1
        if self.damage_type in other.weak:
            bonus = 2
        elif self.damage_type in other.immune:
            bonus = 0
        return self.ap * bonus * self.units

    def attack(self, other):
        damage = self.damage(other)
        dead = damage // other.hp
        other.units -= dead

class Infection(Group):
    pass


class Immune(Group):
    pass


def part1(armies):
    """Solution to part 1"""
    while len(set(type(g) for g in armies)) > 1:
        print(','.join(f"{g.__class__.__name__} {g.units}" for g in armies))
        order = []
        defenders = list(armies)
        for attacker in sorted(armies, key=lambda a: (-a.effective_power, -a.initiative)):
            try:
                defender = max(
                    (d for d in defenders if not isinstance(d, type(attacker))),
                    key=lambda d: (attacker.damage(d), d.effective_power, d.initiative))
                if attacker.damage(defender) == 0:
                    raise ValueError()
                defenders.remove(defender)
                order.append((attacker, defender))
            except ValueError:
                order.append((attacker, None))
        order.sort(key=lambda o: -o[0].initiative)
        for attacker, defender in order:
            if attacker.units <= 0 or defender is None:
                continue
            attacker.attack(defender)
        armies = [a for a in armies if a.units > 0]
    return sum(a.units for a in armies)

def part2(armies):
    """Solution to part 2"""
    pass

def parse(text):
    armies = []

    unit = re.compile(
        r'(?P<units>\d+) units each with (?P<hp>\d+) hit points '
        r'(:?\((?P<attributes>.*?)\) )?'
        r'with an attack that does (?P<ap>\d+) (?P<damage_type>\w+) damage '
        r'at initiative (?P<initiative>\d+)'
    )
    group = None
    for line in text.splitlines():
        if line == '':
            continue
        elif line == "Immune System:":
            group = Immune
        elif line == "Infection:":
            group = Infection
        else:
            m = unit.match(line)
            groups = m.groupdict()
            armies.append(group(**groups))
    return armies

TEST = """Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"""

if __name__ == '__main__':
    assert part1(parse(TEST)) == 5216
    armies = parse(get_input(day=24, year=2018))
    print("Part 1: {}".format(part1(armies)))
    print("Part 2: {}".format(part2(armies)))
    import pdb; pdb.set_trace()
