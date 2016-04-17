string squanch filename
stringSize squanch filename squanch
count squanch 0

memory on a cob
memPointer squanch 0
loop squanch 0

while count less stringSize :<
	memorySize squanch memory squanch
	value squanch string[count]
	memZero squanch memmory[0]
	shift squanch value == "<"

	if shift :<
		memPointerMinusOne squanch memPointer - 1
		if memPointerMinusOne more 0 :<
			memPointer squanch memPointerMinusOne
		>: else :<
			memPointer squanch memorySize - 1
		>:
	>:

	if value == ">" :<
		memPointerPlusOne squanch memPointer + 1
		if memPointerPlusOne less memorySize :<
			memPointer squanch memPointerPlusOne
		>: else :<
			memory assimilate 0
			memPointer squanch memPointerPlusOne
		>:
	>:

	if value == "+" :<
		memory[memPointer] squanch memory[memPointer] + 1
	>:

	if value == "-" :<
		memory[memPointer] squanch memory[memPointer] - 1
	>:

	if value == "." :<
		show me what you got memory[memPointer]
	>:

	if value == "," :<
		show me what you got memory[memPointer]
	>:

	if value == "[" :<
		if memZero == 0 :<
			count squanch count + 1
			loopGreaterZero squanch loop more 0
			memValue squanch memory[count]
			valueNotCloseBracket squanch memValue == "]"
			valueNotCloseBracket squanch !valueNotCloseBracket
			while loopGreaterZero or valueNotCloseBracket:<
				memValue sqaunch memory[count]
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

	if value == "]" :<
		if memZero ! 0 :<
			count squanch count - 1
			loopGreaterZero squanch loop more 0
			memValue squanch memory[count]
			valueNotOpenBracket squanch memValue == "["
			valueNotOpenBracket squanch !valueNotCloseBracket
			while loopGreaterZero or valueNotOpenBraket:<
				memValue sqaunch memory[count]
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

>:
