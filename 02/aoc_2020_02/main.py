import pathlib
import sys
import re

from collections import defaultdict
from dataclasses import dataclass

# 9-18 c: cccccccccwcccccccc

PW_RECORD = re.compile('^(?P<min>\d+)-(?P<max>\d+) (?P<character>\w): (?P<password>.*)$')


@dataclass(eq=False)
class PWRecord:
    range_min: int
    range_max: int
    character: str
    password: str

    def pw_character_count(self):
        character_count = defaultdict(int)

        for c in self.password:
            character_count[c] += 1

        return character_count[self.character]

    def is_valid(self):
        character_count = self.pw_character_count()

        return self.range_min <= character_count and character_count <= self.range_max

    def ruleset2(self):
        condition1 = self.password[self.range_min-1] == self.character
        condition2 = self.password[self.range_max-1] == self.character

        return condition1 ^ condition2


def get_records(filename: pathlib.Path):
    with filename.open(mode='r') as f:
        for line in f.readlines():
            if match := PW_RECORD.match(line):
                yield PWRecord(
                    int(match['min']), int(match['max']),
                    match['character'], match['password'])

def main():
    filename = pathlib.Path(sys.argv[1])

    valid_pwds = [record for record in get_records(filename) if record.is_valid()]
    print(len(valid_pwds))

    valid_rule2 = [record for record in get_records(filename) if record.ruleset2()]
    print(len(valid_rule2))
