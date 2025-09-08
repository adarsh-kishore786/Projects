#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "exit.h"
#include "token.h"
#include "util.h"

enum Token get_token(char ch) {
  switch (ch) {
    case '{': return LEFT_BRACE;
    case '}': return RIGHT_BRACE;
    case '[': return LEFT_BAR;
    case ']': return RIGHT_BAR;
    case ':': return COLON;
    case ',': return COMMA;
    default:
      printf("Error: Invalid character '%c'", ch);
      end_program("", EXIT_INVALID_JSON);
  }
  return EOJ; // should never happen
}

enum Token* get_tokens(const char *text) {
  if (strlen(text) == 0)
    end_program("Empty file", EXIT_INVALID_JSON);

  enum Token *tokens = (enum Token*)malloc((strlen(text)+1)*sizeof(enum Token));
  if (tokens == NULL)
    end_program("A memory allocation error occured", EXIT_NO_MEMORY);

  int i = 0;
  while (i < strlen(text)) {
    char ch = text[i];
    tokens[i] = get_token(ch);
    i++;
  }
  tokens[i] = EOJ;

  return tokens;
}

const char* get_string_token(enum Token *token) {
  switch (*token) {
    case LEFT_BRACE : return "LEFT_BRACE";
    case RIGHT_BRACE: return "RIGHT_BRACE";
    case COLON      : return "COLON";
    case COMMA      : return "COMMA";
    case LEFT_BAR   : return "LEFT_BAR";
    case RIGHT_BAR  : return "RIGHT_BAR";
  }
  return ""; // Should never happen
}

void print_tokens(enum Token **tokens) {
  for (int i = 0; *(*tokens+i) != EOJ; i++) {
    printf("%s\n", get_string_token(*tokens+i));
  }
}
