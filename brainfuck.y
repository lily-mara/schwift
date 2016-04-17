string squanch "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."

stringSize squanch string squanch
count squanch 0

memory on a cob
memory assimilate 0

memPointer squanch 0
loop squanch 0

while count less stringSize :<

	memorySize squanch memory squanch
	if string[count] == "<" :<
		memPointerMinusOne squanch memPointer - 1
		if memPointerMinusOne more 0 :<
			memPointer squanch memPointerMinusOne
		>: else :<
			memPointer squanch memorySize - 1
		>:
	>:

	if string[count] == ">" :<
		memPointerPlusOne squanch memPointer + 1
		if memPointerPlusOne less memorySize :<
			memPointer squanch memPointerPlusOne
		>: else :<
			memory assimilate 0
			memPointer squanch memPointerPlusOne
		>:
	>:

	if string[count] == "+" :<
		memory[memPointer] squanch memory[memPointer] + 1
	>:

	if string[count] == "-" :<
		memory[memPointer] squanch memory[memPointer] - 1
	>:

	if string[count] == "." :<
		show me what you got memory[memPointer]
	>:

	if string[count] == "," :<
		show me what you got memory[memPointer]
	>:

	if string[count] == "[" :<
		if memory[memPointer] == 0 :<
			count squanch count + 1
			loopGreaterZero squanch loop more 0
			memValue squanch memory[count]
			valueNotCloseBracket squanch memValue == "]"
			valueNotCloseBracket squanch !valueNotCloseBracket
			while loopGreaterZero or valueNotCloseBracket :<
				memValue squanch memory[count]
				if memValue == "[" :<
					loop squanch loop + 1
				>:
				if memValue == "]" :<
					loop squanch loop - 1
				>:
				count squanch count + 1
				loopGreaterZero squanch loop more 0
				memValue squanch memory[count]
				valueNotCloseBracket squanch memValue == "]"
				valueNotCloseBracket squanch !valueNotCloseBracket
			>:
		>:
	>:

	if string[count] == "]" :<
		if !memory[memPointer] == 0 :<
			count squanch count - 1
			loopGreaterZero squanch loop more 0
			memValue squanch memory[count]
			valueNotOpenBracket squanch memValue == "["
			valueNotOpenBracket squanch !valueNotCloseBracket
			while loopGreaterZero or valueNotOpenBraket :<
				memValue squanch memory[count]
				if memValue == "[" :<
					loop squanch loop - 1
				>:
				if memValue == "]" :<
					loop squanch loop + 1
				>:
				count squanch count - 1
				loopGreaterZero squanch loop more 0
				memValue squanch memory[count]
				valueNotOpenBracket squanch memValue == "["
				valueNotOpenBracket squanch !valueNotCloseBracket
			>:
			count squanch count - 1
		>:
	>:

	count squanch count + 1
>:
