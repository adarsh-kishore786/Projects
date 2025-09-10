#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "token_type.h"

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

const char* get_string_token_type(enum TokenType *type) {
  switch (*type) {
    case LEFT_BRACE : return "LEFT_BRACE";
    case RIGHT_BRACE: return "RIGHT_BRACE";
    case COLON      : return "COLON";
    case COMMA      : return "COMMA";
    case LEFT_BAR   : return "LEFT_BAR";
    case RIGHT_BAR  : return "RIGHT_BAR";
    case STRING     : return "STRING";
    case BOOLEAN    : return "BOOLEAN";
    case NIL        : return "NIL";
    case NUMBER     : return "NUMBER";
    case EOJ        : return "EOJ";
  }
  return ""; // Should never happen
}
