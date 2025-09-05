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

  public static void main(String[] args) throws IOException {
    if (args.length < 1) {
      System.err.println("Supply a file to be counted!");
      System.exit(1);
    }

    String text = readFile(args[0]);

    System.out.println(text.split("\n")[0]);
  }
}
