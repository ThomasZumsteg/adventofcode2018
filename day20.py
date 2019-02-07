#!/usr/bin/env pipenv run python

import collections

from get_input import get_input

class Point(collections.namedtuple("Point", ["x", "y"])):
    def __repr__(self):
        return f"({self.x}, {self.y})"

    def __add__(self, other):
        if not isinstance(other, Point):
            return NotImplemented
        return type(self)(self.x + other.x, self.y + other.y)

Point.DIRECTIONS = {
    "N": Point(0, 1),
    "S": Point(0, -1),
    "E": Point(1, 0),
    "W": Point(-1, 0)
}


def make_map(paths):
    queue = [(Point(0, 0), paths),]
    room_map = collections.defaultdict(set)
    seen = set()
    while queue:
        state = queue.pop()
        if state in seen:
            continue
        seen.add(state)
        position, paths = state
        if paths == ():
            continue
        elif isinstance(paths[0], Point):
            next_position = position + paths[0]
            room_map[position].add(next_position)
            room_map[next_position].add(position)
            queue.append((next_position, paths[1:]))
        elif isinstance(paths[0], tuple):
            for path in paths[0]:
                queue.append((position, path + paths[1:]))
    return room_map


def part1(paths):
    """Solution to part 1"""
    room_map = make_map(paths)
    queue = [(Point(0, 0), 0)]
    door_counts = {}
    while queue:
        position, doors = queue.pop()
        if position in door_counts:
            continue
        door_counts[position] = doors
        for next_position in room_map[position]:
            queue.append((next_position, doors + 1))
    return max(door_counts.values())


def part2(paths):
    """Solution to part 2"""
    room_map = make_map(paths)
    queue = [(Point(0, 0), 0)]
    door_counts = {}
    while queue:
        position, doors = queue.pop()
        if position in door_counts:
            continue
        door_counts[position] = doors
        for next_position in room_map[position]:
            queue.append((next_position, doors + 1))
    return sum(1 for d in door_counts.values() if d >= 1000)

def parse(text):
    assert text[0] == '^' and text[-1] == '$'
    stack = [[]]
    for char in text[1:-1]:
        if char in Point.DIRECTIONS:
            stack[-1].append(Point.DIRECTIONS[char])
        elif char == '(':
            stack.append([])
            stack.append([])
        elif char == ')':
            sequence = tuple(stack.pop())
            stack[-1].append(sequence)
            sequence = tuple(stack.pop())
            stack[-1].append(sequence)
        elif char == '|':
            sequence = tuple(stack.pop())
            stack[-1].append(sequence)
            stack.append([])
        else:
            raise ValueError(f"WTF is this {char}")
    assert len(stack) == 1
    return tuple(stack.pop())

if __name__ == '__main__':
    assert part1(parse("^WNE$")) == 3
    assert part1(parse("^ENWWW(NEEE|SSE(EE|N))$")) == 10
    assert part1(parse("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$")) == 18
    regex = parse(get_input(day=20, year=2018).strip())
    print("Part 1: {}".format(part1(regex)))
    print("Part 2: {}".format(part2(regex)))
