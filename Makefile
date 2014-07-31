RFLAGS=-g --crate-type=bin -A dead-code -A dead_assignment

all:
	rustc src/main.rs $(RFLAGS)
	rustc src/client.rs $(RFLAGS)

test:
	./main &
	./test/2000/test.py
	killall main

.PHONY: test
