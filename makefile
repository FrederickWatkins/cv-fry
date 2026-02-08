%.vcd:
	cargo test $*
	gtkwave target/$*.vcd