RFLAGS=-g --crate-type=bin -A dead-code -A dead_assignment

all:
	rustc src/main.rs $(RFLAGS)
	rustc src/client.rs $(RFLAGS)

test:
	killall main
	./main &
	./add.sh
	./test.py



.PHONY: test
