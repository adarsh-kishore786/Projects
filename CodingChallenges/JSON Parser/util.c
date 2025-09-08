#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void end_program(const char *message, int error_code) {
  printf("%s\nExiting with error code %d.\n", message, error_code);
  exit(error_code);
}

char* join_string(const char *str1, const char *str2) {
  int l = strlen(str1) + strlen(str2);
  char *str_final = (char*)malloc(sizeof(char) * l);

  sprintf(str_final, "%s%s", str1, str2);
  return str_final;
}
