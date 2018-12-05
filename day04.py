#!/usr/bin/env pipenv run python
"""Solutions to day 4 of Advent of Code"""

import re
import itertools
import collections

from get_input import get_input, line_parser

def part1(guard_log):
    """Solution to part 1"""
    guard_dict = collections.defaultdict(list)
    current_guard = None
    is_asleep = False
    for year, month, day, hour, minute, guard, message in sorted(guard_log, key=lambda g: g[:5]):
        if guard is not None:
            assert not is_asleep
            current_guard = guard
            is_asleep = False
        elif message == "falls asleep":
            assert not is_asleep
            is_asleep = True
            start = (year, month, day, hour, minute)
        elif message == "wakes up":
            assert is_asleep
            is_asleep = False
            end = (year, month, day, hour, minute)
            assert start[:3] == end[:3]
            time = 60*(end[3] - start[3])+end[4]-start[4]
            guard_dict[current_guard].append((time, tuple(start), tuple(end)))
        else:
            raise Exception(f"Unrecognized message {message}")
    most_rested, times = max(guard_dict.items(), key=lambda kv: sum(t[0] for t in kv[1]))
    
    time_counter = collections.defaultdict(int)
    for hour, minute in itertools.product(range(24), range(60)):
        for _, start, end in times:
            if start[3] <= hour < end[3] or (start[3] == end[3] and start[4] <= minute < end[4]):
                time_counter[(hour, minute)] += 1
    result = max(time_counter.items(), key=lambda kv: kv[1])
    return result[0][1] * most_rested

def part2(guard_log):
    """Solution to part 2"""
    guard_dict = collections.defaultdict(list)
    current_guard = None
    is_asleep = False
    for year, month, day, hour, minute, guard, message in sorted(guard_log, key=lambda g: g[:5]):
        if guard is not None:
            assert not is_asleep
            current_guard = guard
            is_asleep = False
        elif message == "falls asleep":
            assert not is_asleep
            is_asleep = True
            start = (year, month, day, hour, minute)
        elif message == "wakes up":
            assert is_asleep
            is_asleep = False
            end = (year, month, day, hour, minute)
            assert start[:3] == end[:3]
            time = 60*(end[3] - start[3])+end[4]-start[4]
            guard_dict[current_guard].append((time, tuple(start), tuple(end)))
        else:
            raise Exception(f"Unrecognized message {message}")
    time_counter = collections.defaultdict(int)
    for hour, minute in itertools.product(range(24), range(60)):
        for guard, times in guard_dict.items():
            for _, start, end in times:
                if start[3] <= hour < end[3] or (start[3] == end[3] and start[4] <= minute < end[4]) and not (hour == end[3] and minute == end[4]):
                    time_counter[(guard, hour, minute)] += 1
    guard, _ = max(time_counter.items(), key=lambda kv: kv[1])
    return guard[0] * guard[2]


def parse(line):
    # format 
    # [1518-11-01 00:00] Guard #10 begins shift
    match = re.match(r'\[(\d+)-(\d+)-(\d+) (\d+):(\d+)](?: Guard #(\d+))? (.*)$', line)
    return [int(g) if (g is not None and g.isdigit()) else g for g in match.groups()]

if __name__ == '__main__':
    LINES = line_parser(get_input(day=4, year=2018), parse=parse)
    print("Part 1: {}".format(part1(LINES)))
    print("Part 2: {}".format(part2(LINES)))
