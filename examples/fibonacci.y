if (argv squanch less 1) :<
	show me what you got "Usage: fibonacci.y [number]"
	show me what you got "Prints the first [number] fibonacci numbers"
>: else :<
	a squanch 0
	b squanch 1

	count squanch 0

	nums on a cob

	while (count less argv[0]) :<
		tmp squanch b
		b squanch (a + b)
		a squanch tmp

		nums assimilate a

		count squanch (count + 1)
	>:

	show me what you got nums
>:
