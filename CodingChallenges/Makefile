all: clean compile run

compile:
	@javac $(pkg)/Main.java

run:
	@java $(pkg)/Main $(arg) || true

clean:
	@rm -rf $(pkg)/*.class
