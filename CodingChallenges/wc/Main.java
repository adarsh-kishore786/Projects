package wc;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;

public class Main {
  public static String readFile(String filePath) throws IOException {
    try (BufferedReader reader = new BufferedReader(new FileReader(filePath))) {

      StringBuilder text = new StringBuilder();
      String line = reader.readLine();

      while (line != null) {
        text.append(line);
        text.append(System.lineSeparator());
        line = reader.readLine();
      }
      return text.toString();

    } catch (IOException e) {
      throw new IOException(e);
    }
  }

  public static void error(String message) {
    System.err.println(message);
    System.exit(1);
  }

  public static void main(String[] args) throws IOException {
    if (args.length < 1) 
      error("Supply a file to be counted!");

    boolean countCharacterOnly = false;
    boolean countLineOnly = false;
    boolean countWordOnly = false;

    String filePath = "";

    for (String arg : args) {
      System.out.println(arg);

      if ((countCharacterOnly && countLineOnly) ||
          (countLineOnly && countWordOnly) ||
          (countCharacterOnly && countWordOnly)) {

        error("Cannot give two or more flags simultaneously!");
      }

      if (arg.equals("-c")) 
        countCharacterOnly = true;
      else if (arg.equals("-l")) 
        countLineOnly = true;
      else if (arg.equals("-w")) 
        countWordOnly = true;
      else if (filePath.isEmpty()) 
        filePath = arg;
    }

    System.out.println(filePath.split("\n")[0]);
    System.out.println(countCharacterOnly);
    System.out.println(countWordOnly);
    System.out.println(countLineOnly);
  }
}
