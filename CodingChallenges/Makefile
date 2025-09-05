all: clean compile run

jar: clean compile _jar

compile:
	@javac $(pkg)/Main.java

run:
	@java $(pkg)/Main $(arg)

clean:
	@rm -rf $(pkg)/*.class

_jar:
	@jar cfe $(jar) $(pkg)/Main $(pkg)/Main.class
	@chmod +x $(jar)
