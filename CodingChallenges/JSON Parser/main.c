#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void readFile(const char *filePath) {
  char ch;

  FILE *fptr = fopen(filePath, "r");

  if (fptr == NULL) {
    printf("%s: File not found.\n", filePath);
    exit(1);
  }

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
