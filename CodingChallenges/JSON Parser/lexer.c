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

int isAtEnd(const char* text) {
  return i >= strlen(text);
}

const char* get_string_token_type(TokenType *type) {
  switch (*type) {
    case LEFT_BRACE : return "LEFT_BRACE";
    case RIGHT_BRACE: return "RIGHT_BRACE";
    case COLON      : return "COLON";
    case COMMA      : return "COMMA";
    case LEFT_BAR   : return "LEFT_BAR";
    case RIGHT_BAR  : return "RIGHT_BAR";
    case EOJ        : return "EOJ";
    case VARIABLE   : return "VARIABLE";
  }
  return ""; // Should never happen
}

Token variable(const char *text) {
  int start = ++i;

  if (isAtEnd(text)) {
    printf("Error: Unterminated string on line %d", line);
    end_program("", EXIT_INVALID_JSON);
  }

  char ch = text[i];

  while (ch != text[start-1]) {
    if (isAtEnd(text) || ch == '\n') {
      printf("Error: Unterminated string on line %d:%d", line+1, column+1);
      end_program("", EXIT_INVALID_JSON);
    }

    ch = text[i++];
    column++;
  }

  char *value = (char*)malloc(sizeof(char)*(i-start));
  strncpy(value, text+start, i-start);
  value[i-start-1] = '\0';
  i--;

  return (Token) { VARIABLE, line, column, value };
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
      line++;
      column = 0;
    } else if (ch == ' ') {
      column++;
    } else if (ch == '\"' || ch == '\'') {
      tokens[count++] = variable(text);
    } else {
      TokenType type = get_simple_token_type(ch, line, column);
      tokens[count++] = (Token) { type, line, column++, "" };
    }
    i++;
  }

  TokenType eoj = EOJ;
  tokens[count] = (Token) { eoj, line+1, 0, "" };

  return tokens;
}

void print_tokens(Token **tokens) {
  int j;
  for (j = 0; (*tokens+j)->token_type != EOJ; j++) {
    Token token = *(*tokens+j);
    printf("%s", get_string_token_type(&token.token_type));

    if (strlen(token.value) > 0)
      printf(": %s", token.value);

    printf("\n");
  }
  printf("%s\n", get_string_token_type(&(*tokens+j)->token_type));
}
