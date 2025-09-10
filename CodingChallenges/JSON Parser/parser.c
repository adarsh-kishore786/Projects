#include <stdio.h>

#include "exit.h"
#include "token.h"
#include "token_type.h"
#include "util.h"

static int i = 0;

TokenType get_type(Token **tokens) {
  return (*tokens + i)->token_type;
}

int is_at_end(Token **tokens) {
  return get_type(tokens) == EOJ ? 1 : 0;
}

void invalid_json(Token **tokens) {
  Token *token = *tokens+i;

  printf("Error: Invalid token %s at line %d:%d", get_string_token_type(&token->token_type), token->line, token->column);
  end_program("", EXIT_INVALID_JSON);
}

void expect(Token **tokens, const TokenType type) {
  if (is_at_end(tokens) || get_type(tokens) != type)
    invalid_json(tokens);

  i++;
}

void object(Token **tokens) {
  expect(tokens, LEFT_BRACE);
  
  expect(tokens, RIGHT_BRACE);
}

void parse(Token **tokens) {
  object(tokens);

  printf("No errors, this is a valid JSON.\n");
  end_program("", EXIT_NORMAL);
}
