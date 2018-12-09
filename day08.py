#!/usr/bin/env pipenv run python
"""Solutions to day 8 of Advent of Code"""

from get_input import get_input, line_parser

class Node:
    """Node in a tree with children and values
    """
    def __init__(self, meta, children):
        self.meta = meta
        self.children = children

    def __repr__(self):
        return f"Node({self.meta}, {self.children})"

    def value(self):
        """Value of the node equal to total if there are no children otherwise
        equal othe sum of the value of children referenced by cardinal index in
        meta
        """
        if self.children == tuple():
            return sum(self.meta)
        total = 0
        for meta in self.meta:
            if 0 < meta <= len(self.children):
                total += self.children[meta-1].value()
        return total

    def total(self):
        """Sum of meta plus the total of all child nodes"""
        return sum(self.meta) + sum(child.total() for child in self.children)

    @classmethod
    def build(cls, records):
        """Returns a tree and the number of items read from records"""
        children = []
        i = 2
        for _ in range(records[0]):
            j, child = cls.build(records[i:])
            i += j
            children.append(child)
        return (i + records[1]), cls(tuple(records[i:i+records[1]]), tuple(children))

def part1(records):
    """Solution to part 1"""
    _, root = Node.build(records)
    return root.total()

def part2(records):
    """Solution to part 2"""
    _, root = Node.build(records)
    return root.value()

if __name__ == '__main__':
    LINES = line_parser(get_input(day=8, year=2018), seperator=' ')
    print("Part 1: {}".format(part1(LINES)))
    print("Part 2: {}".format(part2(LINES)))
