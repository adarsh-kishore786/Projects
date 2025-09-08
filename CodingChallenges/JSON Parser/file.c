#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "exit.h"
#include "util.h"

int get_file_size(FILE *fptr) {
  int prev = ftell(fptr);
  fseek(fptr, 0L, SEEK_END);
  int sz = ftell(fptr);
  fseek(fptr, prev, SEEK_SET); //go back to where we were

  return sz;
}

char* read_file(const char *filePath) {
  char ch;

  FILE *fptr = fopen(filePath, "r");

  if (fptr == NULL) 
    end_program(join_string(filePath, ": File not found"), EXIT_FILE_NOT_FOUND);

  int file_size = get_file_size(fptr);
  char *text = (char*)malloc(sizeof(char) * (file_size+1));
  int c = 0;

  while ((ch = fgetc(fptr)) != EOF) 
    text[c++] = ch;

  text[c] = '\0';

  fclose(fptr);

  return text;
}
