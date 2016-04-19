GRAMMARS=src/grammar/grammar_debug.rs src/grammar/grammar.rs

all : $(GRAMMARS)

src/grammar/grammar_debug.rs : grammar/schwift.rustpeg
	peg-trace $< > $@

src/grammar/grammar.rs : grammar/schwift.rustpeg
	peg $< > $@

clean :
	rm -f $(GRAMMARS)
