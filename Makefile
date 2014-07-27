sourcepath=src/
RFLAGS=-g --crate-type=bin

all: main
	rustc src/main.rs $(RFLAGS)

depends:
	rm Makefile.depends
	make Makefile.depends

Makefile.depends:
	rustc --dep-info Makefile.depends src/main.rs

include Makefile.depends
