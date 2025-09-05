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

  public static int countLines(String text) {
    String lines[] = text.split(System.lineSeparator(), -1);
    return lines.length-1;
  }

  public static void main(String[] args) throws IOException {
    boolean countCharacterOnly = false;
    boolean countLineOnly = false;
    boolean countWordOnly = false;

    String filePath = "";

    for (String arg : args) {
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

    if (filePath.isEmpty())
      error("Usage: ./cwcc <file_name> [-wcl]");

    String text = readFile(filePath);

    if (countLineOnly)
      System.out.println(countLines(text) + " " + filePath);
  }
}
