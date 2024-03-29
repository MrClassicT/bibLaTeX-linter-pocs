import time
import sys
import asyncio
import re
from helper.helper import read_from_file_async, to_file_url
from checks.checks import check_for_missing_fields, check_for_duplicates
from checks.components.entrytypes import get_requirements

start_time = time.perf_counter()

entry_requirements = get_requirements()

entry_pattern = re.compile(r'@(\w+)\{([^,]+),\s*(.*?)\}\n\n', re.DOTALL)
field_pattern = re.compile(r'(\w+)\s*=\s*(?:\{(.*?)\}|(\S+))', re.DOTALL)


# Check if a file path is provided
if len(sys.argv) < 2:
    print('Error: Please provide a .bib filepath.')
    sys.exit(1)

file_path = sys.argv[1]

async def main():
    try:
        file_content = await read_from_file_async(file_path)
    except Exception as err:
        print(f'Error: {err}')
        sys.exit(1)

    entries = [match.groups() for match in entry_pattern.finditer(file_content)]
    formatted_entries = [{
        'type': entry[0],
        'citationName': entry[1].strip(),
        'content': entry[2],
        'position': match.start()
    } for match, entry in zip(entry_pattern.finditer(file_content), entries)]

    # Check for duplicate key names
    check_for_duplicates(formatted_entries)

    # Check for missing fields
    for entry in formatted_entries:
        missing_fields = check_for_missing_fields(entry)
        if missing_fields:
            line_number = file_content[:entry['position']].count('\n') + 1
            file_url = to_file_url(file_path)
            print(f'Anomaly detected in {entry["type"]} entry "{entry["citationName"]}" at position {entry["position"]}: Missing fields - {", ".join(missing_fields)}.\nAt -> {file_url}:{line_number}')
    
    end_time = time.perf_counter()  # End tracking time
    execution_time = end_time - start_time
    print(f"Execution time: {execution_time:.4f} seconds")
    exit(0)


if __name__ ==  '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
