entry_pattern = re.compile(r'@(\w+)\{([^,]+),\s*(.*?)\}\n', re.DOTALL)
field_pattern = re.compile(r'(\w+)\s*=\s*(?:\{(.*?)\}|(\S+))', re.DOTALL)