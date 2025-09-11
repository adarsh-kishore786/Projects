#include <stdio.h>
#include <string.h>

#include "exit.h"
#include "token.h"
#include "token_type.h"
#include "util.h"

static int i = 0;

int object(Token**);
int record(Token**);
int records(Token**);
int key(Token**);
int value(Token**);
int array(Token**);

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

// array -> LEFT_BAR value (COMMA if not last) RIGHT_BAR
int array(Token **tokens) {
  if (!expect(tokens, LEFT_BAR))
    return 0;

  do {
    if (!value(tokens))
      return 0;
  } while (expect(tokens, COMMA));

  return expect(tokens, RIGHT_BAR);
}

// value -> BOOLEAN | STRING | NUMBER | NIL | object | array
int value(Token **tokens) {
  int simple = (
    expect(tokens, BOOLEAN) ||
    expect(tokens, STRING)  ||
    expect(tokens, NUMBER)  ||
    expect(tokens, NIL)
  );

  if (simple == 1)
    return simple;

  const int start = i;
  int res = object(tokens);
  if (!res && i != start)
    return 0;

  if (!res)
    return array(tokens);
  return 1;
}

// record -> key COLON value
int record(Token **tokens) {
  return (
    key(tokens) &&
    expect(tokens, COLON) &&
    value(tokens)
  );
}

// records -> (object | record)* (COMMA if not last)
int records(Token **tokens) {
  do {
    const int start = i;

    int res = object(tokens);
    if (!res && i != start)
      return 0;

    if (!res) {
      res = record(tokens);
      if (!res)
        return 0;
    }    
  } while (expect(tokens, COMMA));

  return 1;
}

// The BNF diagram would be:
// object -> LEFT_BRACE records RIGHT_BRACE
// object -> LEFT_BRACE RIGHT_BRACE
int object(Token **tokens) {
  if (!expect(tokens, LEFT_BRACE))
    return 0;

  const int start = i;

  if (!records(tokens) && i != start)
    return 0;

  return expect(tokens, RIGHT_BRACE);
}

void parse(Token **tokens) {
  int res = object(tokens);
  int code;

  if (!res && i != 0)
    code = 0;
  else
    code = res || array(tokens);

  if (code == 0) {
    Token *token = *tokens+i;

    if (strlen(token->value) > 0) {
      printf("Error: Invalid token %s with value \'%s\' at line %d:%d", 
           get_string_token_type(&token->token_type),
           token->value,
           token->line,
           token->column);
    } else {
      printf("Error: Invalid token %s at line %d:%d", 
           get_string_token_type(&token->token_type),
           token->line,
           token->column);
    }

    end_program("", EXIT_INVALID_JSON);
  }

  printf("No errors, this is a valid JSON.\n");
  end_program("", EXIT_NORMAL);
}
