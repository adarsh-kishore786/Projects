all: compile execute

compile:
	@gcc -o parse *.c

execute:
	@./parse $(arg)

debug:
	@gcc -g -o parse *.c
	@gdb parse

test: compile
	@for f in $(shell ls test/test); do \
		echo $$f; \
		./parse test/test/$$f || true; \
	  echo "\n"; \
	done
