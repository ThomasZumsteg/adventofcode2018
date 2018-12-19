#!/usr/bin/env pipenv run python
"""Solutions to day 15 of Advent of Code"""

import re
import itertools
import collections
from dataclasses import dataclass

from get_input import get_input, line_parser

class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __add__(self, other):
        if not isinstance(other, type(self)):
            return NotImplemented
        return Point(self.x + other.x, self.y + other.y)

    def is_valid(self, point):
        return 0 <= j < len(board) and 0 <= i < len(board[j])

    def __repr__(self):
        return f"Point(x={self.x}, y={self.y})"

    def __hash__(self):
        return hash((self.x, self.y))

    def __eq__(self, other):
        if not isinstance(other, type(self)):
            return NotImplemented
        return self.x == other.x and self.y == other.y

class Unit:
    def __init__(self, x, y):
        self.hp = 200
        self.ap = 3

    def is_enemy(self, other):
        return isinstance(other, Unit) and not isinstance(other, type(self))

class Board(list):
    def get_point(self, pos: Point):
        return self[pos.y][pos.x]

    def swap(self, pos1: Point, pos2: Point):
        self[pos2.y][pos2.x], self[pos1.y][pos1.x] =\
                self[pos1.y][pos1.x], self[pos2.y][pos2.x]

def make_unit_class(char, attack=3, hp=200):
    class Cls(Unit):
        def __init__(self, x, y):
            super().__init__(x, y)
            self.ap = attack
            self.hp = hp 

        def __str__(self):
            return char

        def __repr__(self):
            return f"{type(self).__name__}"

    Cls.__name__ = 'Elf' if char == 'E' else 'Goblin'
    return Cls 

class Wall:
    def __str__(self):
        return '#'

class Space:
    def __str__(self):
        return '.'

def move(pos: Point, board: Board):
    unit = board.get_point(pos)
    if not isinstance(unit, Unit):
        # Can only move units
        raise ValueError(f"unit at {pos} is not a unit")
    queue = [(pos, None)]
    seen = set()
    while queue:
        point, first = queue.pop(0)
        if point in seen:
            continue
        seen.add(point)
        for diff in READ_ORDER:
            new = point + diff
            try:
                space = board.get_point(new)
            except IndexError:
                continue
            if isinstance(space, Space):
                queue.append((new, first or new))
            elif unit.is_enemy(space):
                return first or pos
    # Return the current position of there are no valid moves
    return pos

READ_ORDER = (Point(0, -1), Point(-1, 0), Point(1, 0), Point(0, 1))

def get_unit_order(board): 
    order = []
    for y, row in enumerate(board):
        for x, unit in enumerate(row):
            if isinstance(unit, Unit):
                order.append((Point(x, y), unit))
    return iter(order)


def select_target(board, pos):
    attacker = board.get_point(pos)
    if not isinstance(attacker, Unit):
        raise ValueError(f"unit at {pos} is not a unit")
    t_pos, target = None, None
    for diff in READ_ORDER:
        unit = board.get_point(pos + diff)
        if attacker.is_enemy(unit) and (target is None or target.hp > unit.hp):
            target, t_pos = unit, pos + diff
    return target, t_pos

def fight(board):
    rounds = 0
    while True:
        for pos, attacker in get_unit_order(board):
            if attacker.hp <= 0:
                continue

            remaining = set(type(u) for _, u in get_unit_order(board) if u.hp > 0)
            if len(remaining) <= 1:
                # print(f"\nAfter {rounds}")
                # for pos, attacker in get_unit_order(board):
                #     print(f"{repr(attacker)} @ {pos} has {attacker.hp}")
                # print('\n'.join(''.join(str(u) for u in row) for row in board))
                return list(remaining)[0],\
                    rounds * sum(u.hp for _, u in get_unit_order(board))

            new_pos = move(pos, board)
            board.swap(pos, new_pos)

            target, t_pos = select_target(board, new_pos)
            if target is not None:
                target.hp -= attacker.ap
                if target.hp <= 0:
                    board[t_pos.y][t_pos.x] = Space()
        rounds += 1

def part1(lines):
    """Solution to part 1"""
    elf_class = make_unit_class('E', 3, 200)
    goblin_class = make_unit_class('G', 3, 200)
    board = make_board(lines, elf=elf_class, goblin=goblin_class)
    _, score = fight(board)
    return score

def part2(lines):
    """Solution to part 2"""
    start, end, expanding = 4, 8, True
    goblin_class = make_unit_class('G', 3, 200)
    best_elf_score, elf_ap = None, None
    while start <= end:
        half = (end + start) // 2 if not expanding else end
        elf_class = make_unit_class('E', half, 200)
        board = make_board(lines, elf=elf_class, goblin=goblin_class)
        before_elf_count = sum(1 for row in board for elf in row if isinstance(elf, elf_class))
        winner, score = fight(board)
        # print(f"End with ap = {half}")
        after_elf_count = sum(1 for row in board for elf in row if isinstance(elf, elf_class))
        if expanding and (winner == goblin_class or after_elf_count < before_elf_count):
            start, end = end + 1, end * 2
        elif expanding and (winner == elf_class and after_elf_count == before_elf_count):
            # print(f"Elves ap {half} with a score {score}")
            end = half - 1
            expanding = False
            best_elf_score, elf_ap = score, half
        elif not expanding and (winner == elf_class and after_elf_count == before_elf_count):
            # print(f"Elves ap {half} with a score {score}")
            end = half - 1
            if half < elf_ap:
                best_elf_score, elf_ap = score, half
        elif not expanding and (winner == goblin_class or after_elf_count < before_elf_count):
            start = half + 1
    # print(f"Best is ap {elf_ap} with a score {best_elf_score}")
    return best_elf_score

def make_board(lines, elf=Unit, goblin=Unit):
    board = Board()
    for x, line in enumerate(lines.splitlines()):
        board.append([])
        for y, char in enumerate(list(line)):
            if char == '#':
                board[-1].append(Wall())
            elif char == '.':
                board[-1].append(Space())
            elif char == 'G':
                board[-1].append(goblin(x, y))
            elif char == 'E':
                board[-1].append(elf(x, y))
    return board

sample_boards = [("""#######   
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######""", 27730, 4988),
("""#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######""", 39514, 31284),
("""#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######(""", 27755, 3478),
("""#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######(""", 28944, 6474),
("""#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########""", 18740, 1140),
]

if __name__ == '__main__':
    for board, part1_score, part2_score in sample_boards:
        assert part1_score == part1(board)
        assert part2_score == part2(board)
    board = get_input(day=15, year=2018)
    print("Part 1: {}".format(part1(board)))
    print("Part 2: {}".format(part2(board)))
    # Not 47678
