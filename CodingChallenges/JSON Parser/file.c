#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "exit.h"
#include "util.h"

int getFileSize(FILE *fptr) {
  int prev = ftell(fptr);
  fseek(fptr, 0L, SEEK_END);
  int sz = ftell(fptr);
  fseek(fptr, prev, SEEK_SET); //go back to where we were

  return sz;
}

char* readFile(const char *filePath) {
  char ch;

  FILE *fptr = fopen(filePath, "r");

  if (fptr == NULL) 
    end_program(join_string(filePath, ": File not found"), FILE_NOT_FOUND);

  char *text = (char*)malloc(sizeof(char) * getFileSize(fptr));
  int c = 0;

  while ((ch = fgetc(fptr)) != EOF) 
    text[c++] = ch;

  fclose(fptr);

  return text;
}
