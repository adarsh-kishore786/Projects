all: compile execute

compile:
	@gcc -o parse *.c

execute:
	@./parse $(arg)

debug:
	@gcc -g -o parse *.c
	@gdb parse
