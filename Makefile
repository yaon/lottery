sourcepath=src/
RFLAGS=-g --crate-type=bin

all: main client
	rustc src/main.rs $(RFLAGS)
	rustc src/client.rs $(RFLAGS)

depends:
	rm *.depends
	make Makefile.main.depends Makefile.client.depends

Makefile.main.depends: Makefile
	rustc --dep-info Makefile.main.depends src/main.rs
Makefile.client.depends: Makefile
	rustc --dep-info Makefile.client.depends src/client.rs

include Makefile.main.depends
include Makefile.client.depends
