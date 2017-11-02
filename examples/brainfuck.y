show me what you got! "Please input your brainfuck program: "
portal gun input

show me what you got ""

size squanch (input squanch)
count squanch 0

memory on a cob
i squanch 0
while (i less 30000) :<
	memory assimilate 0
	i squanch (i + 1)
>:
squanch i

memPointer squanch 0
loop squanch 0

while (count less size) :<
	char squanch input[count]

	if (char == "<") :<
		memPointer squanch (memPointer - 1)
	>:

	if (char == ">") :<
		memPointer squanch (memPointer + 1)
	>:

	if (char == "+") :<
		memory[memPointer] squanch (memory[memPointer] + 1)
	>:

	if (char == "-") :<
		memory[memPointer] squanch (memory[memPointer] - 1)
	>:

	if (char == ".") :<
		show me what you got! ascii(memory[memPointer])
	>:

	if (char == ",") :<
		portal gun in
		memory[memPointer] squanch in
		squanch in
	>:

	if (char == "[") :<
		l squanch 0
		if (memory[memPointer] == 0) :<
			count squanch (count + 1)
			while ((l more 0) or !(input[count] == "]")) :<
				if (input[count] == "[") :<
					l squanch (l + 1)
				>:
				if (input[count] == "]") :<
					l squanch (l - 1)
				>:
				count squanch (count + 1)
			>:
		>:
	>:

	if (char == "]") :<
		l squanch 0
		if !(memory[memPointer] == 0) :<
			count squanch (count - 1)
			while ((l more 0) or !(input[count] == "[")) :<
				if (input[count] == "]") :<
					l squanch (l + 1)
				>:
				if (input[count] == "[") :<
					l squanch (l - 1)
				>:
				count squanch (count - 1)
			>:
			count squanch (count - 1)
		>:
	>:

	count squanch (count + 1)
>:

show me what you got ""
