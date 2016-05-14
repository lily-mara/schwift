#!/usr/bin/env python3

import sys

indent = 0

for line in sys.stdin:
	whitespace = '    ' * indent

	line = line.strip()
	if 'Attempting' in line:
		print(whitespace + line)
		indent += 1
	elif 'Matched' in line or 'Failed' in line:
		indent -= 1
		whitespace = '    ' * indent
		print(whitespace + line)
		print()
