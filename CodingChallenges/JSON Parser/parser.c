#include <stdio.h>

#include "exit.h"
#include "token.h"
#include "token_type.h"
#include "util.h"

static int i = 0;

int object(Token**);
int record(Token**);
int key(Token**);
int value(Token**);

TokenType get_type(Token **tokens) {
  return (*tokens + i)->token_type;
}

int is_at_end(Token **tokens) {
  return get_type(tokens) == EOJ ? 1 : 0;
}

TokenType peek(Token **tokens) {
  if (is_at_end(tokens))
    return EOJ;

  return (*tokens + i + 1)->token_type;
}

int expect(Token **tokens, const TokenType type) {
  if (is_at_end(tokens) || get_type(tokens) != type)
    return 0;

  i++;
  return 1;
}

// key -> STRING
int key(Token **tokens) {
  return expect(tokens, STRING);
}

// value -> BOOLEAN | STRING | NUMBER | NIL
int value(Token **tokens) {
  return (
    expect(tokens, BOOLEAN) ||
    expect(tokens, STRING)  ||
    expect(tokens, NUMBER)  ||
    expect(tokens, NIL)
  );
}

// record -> key COLON value (COMMA if last)
int record(Token **tokens) {
  int res = 1;
  do {
    res = res && (
      key(tokens) &&
      expect(tokens, COLON) &&
      value(tokens)
    );
  } while (expect(tokens, COMMA));

  return res;
}

// line -> object | record | empty
int line(Token **tokens) {
  const int start = i;

  if (object(tokens) == 1)
    return 1;
  if (i != start)
    return 0;

  if (record(tokens) == 1)
    return 1;
  if (i != start)
    return 0;

  // means it is empty
  return 1;
}

// The BNF diagram would be:
// object -> LEFT_BRACE line RIGHT_BRACE
int object(Token **tokens) {
  return ( 
    expect(tokens, LEFT_BRACE) &&
    line(tokens) &&
    expect(tokens, RIGHT_BRACE)
  );
}

void parse(Token **tokens) {
  int code = object(tokens);

  if (code == 0) {
    Token *token = *tokens+i;

    printf("Error: Invalid token %s with value \'%s\' at line %d:%d", 
           get_string_token_type(&token->token_type),
           token->value,
           token->line,
           token->column);

    end_program("", EXIT_INVALID_JSON);
  }

  printf("No errors, this is a valid JSON.\n");
  end_program("", EXIT_NORMAL);
}
