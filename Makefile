RFLAGS=-g --crate-type=bin

all:
	rustc src/main.rs $(RFLAGS)
	rustc src/client.rs $(RFLAGS)
