#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void getFileSize(FILE *fptr) {
  int prev=ftell(fp);
  fseek(fp, 0L, SEEK_END);
  int sz=ftell(fp);
  fseek(fp,prev,SEEK_SET); //go back to where we were

  return sz;
}

char* readFile(const char *filePath) {
  char ch;

  FILE *fptr = fopen(filePath, "r");

  if (fptr == NULL) {
    printf("%s: File not found.\n", filePath);
    exit(1);
  }

  char text[getFileSize(fptr)];
  int c = 0;

  while ((ch = fgetc(fptr)) != EOF && c < len(text))
    text[c++] = ch;

  fclose(fptr);
  return text;
}

int main(int argc, const char **argv) {
  if (argc < 2) {
    printf("Usage: ./parse <file-name>\n");
    exit(0);
  }

  char* text = readFile(argv[1]);
  printf("%s\n", text);
}
