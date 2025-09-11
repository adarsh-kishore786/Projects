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

void lexer_error(const char *message) {
  printf(message, line+1, column+2);
  end_program("", EXIT_INVALID_JSON);
}

int isAtEnd(const char *text) {
  return i >= strlen(text);
}

Token process_string(const char *text) {
  const int start = ++i;
  const int start_in_line = column;

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

  return (Token) { STRING, line+1, start_in_line+1, value };
}

Token process_alpha(const char *text, char *expect, TokenType return_type) {
  const char* variable_error_message = "Error: Malformed variable at line %d:%d";

  const int start = i++;
  const int start_in_line = column;
  
  column++;
  int c = 0;

  while (++c < strlen(expect)) {
    if (isAtEnd(text) || expect[c] != text[i++]) 
      lexer_error(variable_error_message);

    column++;
  }

  if (!isAtEnd(text) && isalnum(text[i])) 
    lexer_error(variable_error_message);

  i--;

  return (Token) { return_type, line+1, start_in_line+1, expect[0] == 'n' ? "": expect }; // don't print null
}

Token process_digit(const char* text) {
  const int start = i++;
  const int start_in_line = column;

  char ch = text[i];
  const char *error_message = "Error: Malformed number at line %d:%d";
  int dotAppeared = 0;

  while (!isAtEnd(text) && (isdigit(ch) || ch == '.')) {
    if (ch == '.') {
      if (dotAppeared == 1)
        lexer_error(error_message);

      dotAppeared = 1;
    }

    ch = text[++i];
    column++;
  }

  char *value = (char*)malloc(sizeof(char)*(i-start+1));
  strncpy(value, text+start, i-start);
  value[i-start] = '\0'; 
  i--;

  return (Token) { NUMBER, line+1, start_in_line+1, value };
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
      tokens[count++] = process_string(text);
    } else if (ch == 't') {
      tokens[count++] = process_alpha(text, "true", BOOLEAN);
    } else if (ch == 'f') {
      tokens[count++] = process_alpha(text, "false", BOOLEAN);
    } else if (ch == 'n') {
      tokens[count++] = process_alpha(text, "null", NIL);
    } else if (isdigit(ch) || ch == '-') {
      tokens[count++] = process_digit(text);
    } else {
      TokenType type = get_simple_token_type(ch, line, column);
      tokens[count++] = (Token) { type, line+1, ++column, "" };
    }
    i++;
  }

  TokenType eoj = EOJ;
  tokens[count] = (Token) { eoj, line+1, 1, "" };

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
  printf("%s\n\n", get_string_token_type(&(*tokens+j)->token_type));
}
