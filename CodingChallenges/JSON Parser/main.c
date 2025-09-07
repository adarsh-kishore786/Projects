#include <stdio.h>
#include <stdlib.h>

#include "file.h"
#include "util.h"

const extern int EXIT_NORMAL;

int main(int argc, const char **argv) {
  if (argc < 2) 
    error("Usage: ./parse <file-name>", EXIT_NORMAL);

  char* text = readFile(argv[1]);
  printf("%s\n", text);
  free(text);
}
