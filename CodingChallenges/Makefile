all: clean compile run

compile:
	javac $(pkg)/Main.java

run:
	java $(pkg)/Main $(pkg)/$(arg)

clean:
	rm -rf $(pkg)/*.class
