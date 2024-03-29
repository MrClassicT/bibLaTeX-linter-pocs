#from components.regex import field_pattern
from checks.components.entrytypes import get_requirements
import re

entry_requirements = get_requirements()

entry_pattern = re.compile(r'@(\w+)\{([^,]+),\s*(.*?)\}\n\n', re.DOTALL)
field_pattern = re.compile(r'(\w+)\s*=\s*(?:\{(.*?)\}|(\S+))', re.DOTALL)


def check_for_missing_fields(entry):
    fields = {}
    for match in field_pattern.finditer(entry['content']):
        key, value1, value2 = match.groups()
        fields[key] = value1 or value2

    missing_fields = []
    if entry['type'] in entry_requirements:
        for field in entry_requirements[entry['type']]['required']:
            if field not in fields:
                missing_fields.append(field)

    return missing_fields

def check_for_duplicates(entries):
    citation_names = [entry['citationName'] for entry in entries]
    duplicates = {name for name in citation_names if citation_names.count(name) > 1}
    if duplicates:
        plural = len(duplicates) > 1
        print(f'Caution: {"A duplicate key" if not plural else "Duplicate keys"} {"has" if not plural else "have"} been found: {", ".join(duplicates)}!')
        exit(1)
