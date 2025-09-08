#include <stdio.h>
#include <stdlib.h>

#include "exit.h"
#include "file.h"
#include "util.h"

int main(int argc, const char **argv) {
  if (argc < 2) 
    end_program("Usage: ./parse <file-name>", NORMAL);

  char* text = readFile(argv[1]);
  printf("%s\n", text);
  free(text);
}
