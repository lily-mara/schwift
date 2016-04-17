src/schwift_grammar.rs : grammar/schwift.rustpeg
	peg $< > $@

clean :
	rm src/schwift_grammar.rs
