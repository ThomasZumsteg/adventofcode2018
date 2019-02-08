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

def map_distances(room_map):
    queue = [(Point(0, 0), 0)]
    door_counts = {}
    while queue:
        position, doors = queue.pop()
        if position in door_counts:
            continue
        door_counts[position] = doors
        for next_position in room_map[position]:
            queue.append((next_position, doors + 1))
    return door_counts

def part1(room_map):
    """Solution to part 1"""
    return max(map_distances(room_map).values())

def part2(room_map):
    """Solution to part 2"""
    return sum(1 for d in map_distances(room_map).values() if d >= 1000)

def parse(chars):
    chars = chars.strip('$^')
    stack = []
    location = Point(0, 0)
    door_map = collections.defaultdict(set)
    while chars:
        char, chars = chars[0], chars[1:]
        if char in Point.DIRECTIONS:
            new_location = location + Point.DIRECTIONS[char]
            door_map[new_location].add(location)
            door_map[location].add(new_location)
            location = new_location
        elif char == '(':
            stack.append(location)
        elif char == ')':
            location = stack.pop()
        elif char == '|':
            location = stack[-1]
        else:
            raise ValueError(f"What is this {char}")
    if stack != []:
        raise RuntimeError("Not a valid REGEX, paenthesis do not match")
    return door_map

if __name__ == '__main__':
    assert part1(parse("^WNE$")) == 3
    assert part1(parse("^ENWWW(NEEE|SSE(EE|N))$")) == 10
    assert part1(parse("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$")) == 18
    mapping = parse(get_input(day=20, year=2018).strip())
    print("Part 1: {}".format(part1(mapping)))
    print("Part 2: {}".format(part2(mapping)))
