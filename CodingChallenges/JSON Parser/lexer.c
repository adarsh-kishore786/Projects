#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "exit.h"
#include "token.h"
#include "token_type.h"
#include "util.h"

static int i = 0;
static int line = 0;
static int column = 0;

const char* get_string_token_type(TokenType *type) {
  switch (*type) {
    case LEFT_BRACE : return "LEFT_BRACE";
    case RIGHT_BRACE: return "RIGHT_BRACE";
    case COLON      : return "COLON";
    case COMMA      : return "COMMA";
    case LEFT_BAR   : return "LEFT_BAR";
    case RIGHT_BAR  : return "RIGHT_BAR";
    case EOJ        : return "EOJ";
  }
  return ""; // Should never happen
}

TokenType get_simple_token_type(char ch, int line, int column) {
  switch (ch) {
    case '{': return LEFT_BRACE;
    case '}': return RIGHT_BRACE;
    case '[': return LEFT_BAR;
    case ']': return RIGHT_BAR;
    case ':': return COLON;
    case ',': return COMMA;
    default:
      printf("Error: Invalid character '%c' at line %d:%d", ch, line+1, column+1);
      end_program("", EXIT_INVALID_JSON);
  }
  return EOJ; // should never happen
}

Token* get_tokens(const char *text) {
  if (strlen(text) == 0)
    end_program("Empty file", EXIT_INVALID_JSON);

  Token *tokens = (Token*)malloc((strlen(text)+1)*sizeof(Token));
  if (tokens == NULL)
    end_program("A memory allocation error occured", EXIT_NO_MEMORY);

  int count = 0;

  while (i < strlen(text)) {
    char ch = text[i];

    if (ch == '\n') {
      i++;
      line++;
      column = 0;
    } else if (ch == ' ') {
      i++;
      column++;
    } else {
      TokenType type = get_simple_token_type(ch, line, column);
      tokens[count++] = (Token) { type, line, column++, get_string_token_type(&type) };
      i++;
    }
  }

  TokenType eoj = EOJ;
  tokens[count] = (Token) { eoj, line+1, 0, get_string_token_type(&eoj) };

  return tokens;
}

void print_tokens(Token **tokens) {
  int i;
  for (i = 0; (*tokens+i)->token_type != EOJ; i++) {
    printf("%s\n", get_string_token_type(&(*tokens+i)->token_type));
  }
  printf("%s\n", get_string_token_type(&(*tokens+i)->token_type));
}
