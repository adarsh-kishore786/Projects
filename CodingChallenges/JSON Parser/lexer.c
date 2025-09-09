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

const char* variable_error_message = "Error: Malformed variable at line %d:%d";

void lexer_error(const char *message) {
  printf(message, line+1, column+1);
  end_program("", EXIT_INVALID_JSON);
}

int isAtEnd(const char *text) {
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
    case STRING     : return "STRING";
    case BOOLEAN    : return "BOOLEAN";
    case NIL        : return "NIL";
    case EOJ        : return "EOJ";
  }
  return ""; // Should never happen
}

Token string(const char *text) {
  int start = ++i;
  const char* error_message = "Error: Unterminated string on line %d:%d";

  if (isAtEnd(text)) 
    lexer_error(error_message);

  char ch = text[i];

  while (ch != text[start-1]) {
    if (isAtEnd(text) || ch == '\n') 
      lexer_error(error_message);

    ch = text[i++];
    column++;
  }

  char *value = (char*)malloc(sizeof(char)*(i-start));
  strncpy(value, text+start, i-start);
  value[i-start-1] = '\0';
  i--;

  return (Token) { STRING, line, column, value };
}

Token process_alpha(const char *text, char *expect, TokenType return_type) {
  int start = i++;
  
  column++;
  int c = 0;

  while (++c < strlen(expect)) {
    if (isAtEnd(text) || expect[c] != text[i++]) 
      lexer_error(variable_error_message);

    column++;
  }

  if (!isAtEnd(text) && isalnum(text[i])) {
    column++;
    lexer_error(variable_error_message);
  }
  i--;

  return (Token) { return_type, line, column, expect[0] == 'n' ? "": expect }; // don't print null
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
      tokens[count++] = string(text);
    } else if (ch == 't') {
      tokens[count++] = process_alpha(text, "true", BOOLEAN);
    } else if (ch == 'f') {
      tokens[count++] = process_alpha(text, "false", BOOLEAN);
    } else if (ch == 'n') {
      tokens[count++] = process_alpha(text, "null", NIL);
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
