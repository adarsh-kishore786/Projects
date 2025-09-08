#ifndef TOKEN_H
#define TOKEN_H

#include "token_type.h"

typedef struct Token {
  enum TokenType token_type;
  int line;
  int column;
  char* value;
} Token;

#endif
