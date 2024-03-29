import aiofiles
import urllib.parse
import os

async def read_from_file_async(file_name):
    try:
        async with aiofiles.open(file_name, mode='r', encoding='utf-8') as file:
            return await file.read()
    except Exception as err:
        print(err)
        exit(1)  # It might be better to use sys.exit(1) after importing sys

def to_file_url(file_path):
    absolute_path = os.path.abspath(file_path)
    absolute_path = urllib.parse.quote(absolute_path)
    if os.name == 'nt':
        # Windows path fix
        absolute_path = '/' + absolute_path.replace('\\', '/')
        return f'file://{absolute_path}'
    else:
        # Unix path
        return f'file://{absolute_path}'
