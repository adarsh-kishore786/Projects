#include <ctype.h>
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

Token variable(const char *text) {
  return (Token) {};
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

  TokenType last_token_type = EOJ;
  Token *tokens = (Token*)malloc((strlen(text)+1)*sizeof(Token));
  if (tokens == NULL)
    end_program("A memory allocation error occured", EXIT_NO_MEMORY);

  int count = 0;

  while (i < strlen(text)) {
    char ch = text[i];

    if (ch == '\n') {
      line++;
      column = 0;
    } else if (ch == ' ') {
      column++;
    } else if (isalnum(ch) || ch == '_' || ch == '\"' || ch == '\'') {
      tokens[count++] = variable(text);
    } else {
      last_token_type = get_simple_token_type(ch, line, column);
      tokens[count++] = (Token) { last_token_type, line, column++, get_string_token_type(&last_token_type) };
    }
    i++;
  }

  TokenType eoj = EOJ;
  tokens[count] = (Token) { eoj, line+1, 0, get_string_token_type(&eoj) };

  return tokens;
}

void print_tokens(Token **tokens) {
  int j;
  for (j = 0; (*tokens+j)->token_type != EOJ; j++) {
    printf("%s\n", get_string_token_type(&(*tokens+j)->token_type));
  }
  printf("%s\n", get_string_token_type(&(*tokens+j)->token_type));
}
