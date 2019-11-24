"""Module for getting the input for advent of code"""

import logging
import os
import requests

SESSION_FILE = '.token'
AOC_URL = "http://adventofcode.com"
LOG = logging.getLogger(__name__)

def get_input(day, year):
    """Get the input for a specific day and year from advent of code"""
    file_name = f".AoC-{year:04}-{day:02}.tmp"
    try:
        with open(file_name, 'r') as file_handle:
            return file_handle.read()
    except FileNotFoundError:
        LOG.warning("Attempting to download file from AOC")

    url = f"{AOC_URL}/{year}/day/{day}/input"
    with open(SESSION_FILE) as fh:
        session = fh.read().strip()
    response = requests.get(url, cookies=dict(session=session))
    if not response.ok:
        raise RuntimeError(f"Could not get {url}: {response.status_code}: {response.reason}")
    with open(file_name, 'w') as file_handle:
        file_handle.write(response.text)
    return response.text

def line_parser(text, parse=int, seperator='\n'):
    """Parse lines, usually into base 10 integers by lines"""
    return [parse(item) for item in text.split(seperator) if item != '']
