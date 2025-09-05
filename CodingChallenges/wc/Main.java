package wc;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;

public class Main {
  static String readFile(String filePath) {
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
      error("cwcc: " + filePath + ": No such file");
    }
    return "";
  }

  static void error(String message) {
    System.err.println(message);
    System.exit(1);
  }

  static int countLines(String text) {
    String lines[] = text.split(System.lineSeparator(), -1);
    return lines.length-1;
  }

  static int countWords(String text) {
    String words[] = text.split("[\\s]+", -1);
    return words.length-1;
  }

  static int countCharacters(String text) {
    return text.length();
  }

  public static void main(String[] args) {
    boolean countCharacter = false;
    boolean countLine = false;
    boolean countWord = false;

    String filePath = "";

    for (String arg : args) {
      if (arg.equals("-c")) 
        countCharacter = true;
      else if (arg.equals("-l")) 
        countLine = true;
      else if (arg.equals("-w")) 
        countWord = true;
      else if (arg.equals("-cw") || arg.equals("-wc")) {
        countCharacter = true;
        countWord = true;
      }
      else if (arg.equals("-lw") || arg.equals("-wl")) {
        countLine = true;
        countWord = true;
      }
      else if (arg.equals("-cl") || arg.equals("-lc")) {
        countCharacter = true;
        countLine = true;
      }
      else if (filePath.isEmpty()) 
        filePath = arg;
    }

    if (filePath.isEmpty())
      error("Usage: ./cwcc <file_name> [-wcl]");

    String text = readFile(filePath);
    String response = "__LINES__ __WORDS__ __CHARS__ " + filePath;

    if (!countCharacter && !countWord && !countLine) {
      countCharacter = true;
      countWord = true;
      countLine = true;
    }

    if (countLine)
      response = response.replace("__LINES__", "" + countLines(text));

    if (countWord)
      response = response.replace("__WORDS__", "" + countWords(text));

    if (countCharacter)
      response = response.replace("__CHARS__", "" + countCharacters(text));

    response = response.replaceAll("__[A-Z]+__", "").trim();
    System.out.println(response);
  }
}
