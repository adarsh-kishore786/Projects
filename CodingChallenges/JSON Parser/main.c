#include <stdio.h>
#include <stdlib.h>

void readFile(const char *filePath) {
  char ch;

  FILE *fptr = fopen(filePath, "r");

  while ((ch = fgetc(fptr)) != EOF)
    printf("%c", ch);
  printf("\n");

  fclose(fptr);
}

int main(int argc, const char **argv) {
  if (argc < 2) {
    printf("Usage: ./parse <file-name>\n");
    exit(0);
  }

  readFile(argv[1]);
}
