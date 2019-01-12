#!/usr/bin/env pipenv run python
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

    def __repr__(self):
        return f"Point(x={self.x}, y={self.y})"

    def __hash__(self):
        return hash((self.x, self.y))

    def __eq__(self, other):
        if not isinstance(other, type(self)):
            return NotImplemented
        return self.x == other.x and self.y == other.y

class BoardItem:
    def __init__(self, board=None, pos=None):
        self.board = board 
        self.pos = pos

    def __str__(self):
        return self.__class__.char
    
class Wall(BoardItem):
    char = '#'

class Space(BoardItem):
    char = '.'

class Unit(BoardItem):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._hp = type(self).cls_hp
        self.ap = type(self).cls_ap

    @property
    def hp(self):
        return self._hp

    @hp.setter
    def hp(self, other):
        self._hp = other
        if self._hp <= 0:
            self.board[self.pos] = Space()
            if self.on_death:
                raise self.on_death(repr(self))

    def is_enemy(self, other):
        return isinstance(other, Unit) and not isinstance(other, type(self))

    def find_defender(self):
        defender = None
        for diff in [self.board[self.pos + diff] for diff in self.board.READ_ORDER]:
            if self.is_enemy(diff):
                if not defender or diff.hp < defender.hp:
                    defender = diff
        return defender

    @classmethod
    def make_unit_class(cls, char, attack=3, hp=200, on_death=None):
        unit_type = "Elf" if char == "E" else "Goblin"
        class Cls(cls):
            def __repr__(self):
                return f"{unit_type}({repr(self.pos)}, attack={self.ap}, hp={self.hp})"
        Cls.char = char
        Cls.cls_ap = attack
        Cls.cls_hp = hp
        Cls.on_death = on_death
        return Cls 


class Board:
    READ_ORDER = (Point(0, -1), Point(-1, 0), Point(1, 0), Point(0, 1))

    def __init__(self):
        self._rows = []
        self.round = 0

    class NoEnemies(Exception):
        pass

    @classmethod
    def make_board(cls, lines, mapping):
        board = cls()
        for y, line in enumerate(lines.splitlines()):
            for x, char in enumerate(list(line)):
                board[Point(x, y)] = mapping[char]()
        board.unit_order = board.units
        board.attacker = board.unit_order.pop(0)
        return board

    def __getitem__(self, pos):
        if isinstance(pos, Point):
            return self._rows[pos.y][pos.x]
        return NotImplemented

    def __setitem__(self, point, value):
        if not isinstance(point, Point):
            return NotImplemented
        elif not isinstance(value, BoardItem):
            return NotImplemented
        while point.y >= len(self._rows):
            self._rows.append([])
        while point.x >= len(self._rows[point.y]):
            self._rows[point.y].append(None)
        self._rows[point.y][point.x] = value
        value.board = self
        value.pos = point

    def __iter__(self):
        return iter(self._rows)

    @property
    def units(self):
        return [u for row in self for u in row if isinstance(u, Unit)]

    def find_path(self, attacker):
        queue = []
        for d in self.READ_ORDER:
            if attacker.is_enemy(self[attacker.pos+d]):
                return attacker.pos
            elif isinstance(self[attacker.pos+d], Space):
                queue.append((attacker.pos+d,attacker.pos+d))
        enemies = [unit for unit in self.units if attacker.is_enemy(unit)]
        if not enemies:
            raise type(self).NoEnemies("No enemies")
        goals = set()
        for unit in enemies:
            for diff in self.READ_ORDER:
                space = unit.pos + diff
                if isinstance(self[space], Space):
                    goals.add(space)
        seen = set((attacker.pos,))
        while queue:
            first, current = queue.pop(0)
            if current in seen:
                continue
            seen.add(current)
            if current in goals:
                return first
            for diff in type(self).READ_ORDER:
                new = current + diff
                try:
                    square = self[new]
                except IndexError:
                    continue
                if isinstance(square, Space):
                    queue.append((first, new))

    def __repr__(self):
        representation = [f"Round {self.round}"]
        representation.extend(''.join(str(u) for u in row) for row in self)
        representation.extend(repr(u) for u in self.units)
        return '\n'.join(representation)

    def play_round(self):
        for attacker in self.units:
            if attacker.hp <= 0:
                continue

            move = self.find_path(attacker)
            if move:
                self[attacker.pos], self[move] = self[move], self[attacker.pos]

            defender = attacker.find_defender()
            if defender:
                defender.hp -= attacker.ap

        self.round += 1


def part1(lines):
    """Solution to part 1"""
    board = Board.make_board(lines, {
        'E': Unit.make_unit_class('E'),
        'G': Unit.make_unit_class('G'),
        '#': Wall,
        '.': Space})
    while True:
        try:
            board.play_round()
        except Board.NoEnemies:
            break
    return sum(u.hp for u in board.units) * board.round

def part2(lines):
    """Solution to part 2"""
    start, end, expanding, best_ap = 4, 8, True, None
    goblin_class = Unit.make_unit_class('G')
    class DeadElf(Exception):
        pass
    best_score = None
    while start <= end:
        elf_ap = (start + end) // 2 if not expanding else end
        elf_class = Unit.make_unit_class('E', elf_ap, 200, DeadElf)
        print(f"Testing {start} -> {end}, {elf_class.cls_ap}")
        board = Board.make_board(lines, {'E': elf_class, 'G': goblin_class, '#': Wall, '.': Space})
        try:
            while True:
                board.play_round()
        except DeadElf:
            print("Dead Elf")
            if expanding:
                start, end = end + 1, end * 2
            else:
                start = elf_ap + 1
                expanding = False
        except Board.NoEnemies:
            print("Elf Win")
            score = sum(u.hp for u in board.units) * board.round
            if best_score is None or best_ap > elf_ap:
                best_score, best_ap = score, elf_ap
                print(f"Best score {best_score}:{elf_class.cls_ap}")
            if expanding:
                end = end - 1
                expanding = False
            else:
                end = elf_ap - 1
    return best_score

sample_boards = [("""#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######""", 27730, 4988),
("""#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######""", 36334, None),
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
#######""", 27755, 3478),
("""#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######""", 28944, 6474),
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
        assert part2_score is None or part2_score == part2(board)
    board = get_input(day=15, year=2018)
    print("Part 1: {}".format(part1(board)))
    print("Part 2: {}".format(part2(board)))
    # Not 47678 46140
    # Is 46784
