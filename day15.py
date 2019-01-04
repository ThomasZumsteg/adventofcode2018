
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
    def __init__(self, pos, board):
        self.pos = pos
        self.board = board

    def __str__(self):
        return self.__class__.char

class Wall(BoardItem):
    char = '#'

class Space(BoardItem):
    char = '.'

class Unit(BoardItem):
    def __init__(self, board, pos):
        super().__init__(board, pos)
        self._hp = type(self).hp

    @property
    def hp(self):
        return self._hp

    @hp.setter
    def hp(self, other):
        self._hp = other
        if self._hp < 0 and self.on_death:
            raise self.on_death(repr(self))

    def is_enemy(self, other):
        return isinstance(other, Unit) and not isinstance(other, type(self))

    @classmethod
    def make_unit_class(cls, char, attack=3, hp=200, on_death=None):
        unit_type = "Elf" if char == "E" else "Goblin"
        class Cls(cls):
            def __repr__(self):
                return f"{unit_type}({repr(self.pos)}, attack={self.ap}, hp={self.hp})"
        Cls.char = char
        Cls.ap = attack
        Cls.hp = hp
        Cls.on_death = on_death
        return Cls 
    
    def attack(self):
        defenders = []
        for diff in self.board.READ_ORDER:
            defender = self.board[self.pos + diff]
            if self.is_enemy(defender):
                defenders.append(defender)
        if not defenders:
            return
        defenders.sort(key=lambda d: d.hp)
        defender = defenders[0]
        defender.hp -= self.ap
        if defender.dead:
            self.board.remove(defender)
            return defender

class Board:
    READ_ORDER = (Point(0, -1), Point(-1, 0), Point(1, 0), Point(0, 1))

    def __init__(self, rows, round=0, unit_order=None):
        self._rows = tuple(tuple(row) for row in rows)
        self.round = round
        if not unit_order:
            unit_order = self.units
            self.round += 1
        self.attacker, *self.unit_order = unit_order

    @classmethod
    def make_board(cls, lines, mapping):
        rows = []
        for y, line in enumerate(lines.splitlines()):
            rows.append([])
            for x, char in enumerate(list(line)):
                rows[-1].append(mapping[char](Point(x, y), board))
        return cls(rows) 

    def __getitem__(self, pos):
        if isinstance(pos, Point):
            return self._rows[pos.y][pos.x]
        return NotImplemented

    def __iter__(self):
        return iter(self._rows)

    @property
    def units(self):
        return tuple(u for row in self for u in row if isinstance(u, Unit))

    def find_path(self, attacker):
        queue = [(attacker.pos, )]
        seen = set()
        while queue:
            path = queue.pop(0)
            point = path[-1]
            if point in seen:
                continue
            seen.add(point)
            for diff in type(self).READ_ORDER:
                new = point + diff
                try:
                    square = self[new]
                except IndexError:
                    continue
                if isinstance(square, Space):
                    queue.append(path + (new,))
                elif attacker.is_enemy(square):
                    return path + (new,)
        return tuple()

    def __repr__(self):
        representation = [f"Round {self.round}"]
        representation.extend(''.join(str(u) for u in row) for row in self)
        representation.extend(repr(u) for u in self.units)
        return '\n'.join(representation)

    def play(self):
        path = self.find_path(self.attacker)
        rows = [list(row) for row in self]
        if len(path) > 2:
            p1, p2 = path[0], path[1]
            rows[p2.y][p2.x], rows[p1.y][p1.x] = self[p1], self[p2]
            path = path[1:]
        if len(path) == 2:
            p1, p2, *_ = path
            rows[p2.y][p2.x].hp -= rows[p1.y][p1.x].ap
        board = type(self)(rows, round=self.round, unit_order=self.unit_order)
        import pdb; pdb.set_trace()
        return board


def part1(lines):
    """Solution to part 1"""
    board = Board.make_board(lines, {
        'E': Unit.make_unit_class('E'),
        'G': Unit.make_unit_class('G'),
        '#': Wall,
        '.': Space})
    while len(set(type(u) for u in board.units)) > 1:
        board = board.play()
    return sum(u.hp for u in board.units) * board.round

def part2(lines):
    """Solution to part 2"""
    start, end, expanding, best_ap = 4, 8, True, None
    goblin_class = Unit.make_unit_class('G')
    while start < end:
        elf_ap = (start + end) // 2 if not expanding else end
        elf_class = Unit.make_unit_class('E', elf_ap, 300, DeadElf)
        # print(f"Testing {start} -> {end}, {elf_class.ap}")
        board = Board.make_board(lines, {'E': elf_class, 'G': goblin_class, '#': Wall, '.': Space})
        try:
            while len(set(type(u) for u in board.units)) > 1:
                board = board.play()
        except DeadElf:
            if expanding:
                start, end = end + 1, end * 2
            else:
                start = elf_ap + 1
                expanding = False
        else:
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
        assert part2_score == part2(board)
    board = get_input(day=15, year=2018)
    print("Part 1: {}".format(part1(board)))
    print("Part 2: {}".format(part2(board)))
    # Not 47678 46140
    # Is 46784
