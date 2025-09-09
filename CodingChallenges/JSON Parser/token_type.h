#ifndef TOKEN_TYPE_H
#define TOKEN_TYPE_H

typedef enum TokenType {
  // single characters
  LEFT_BRACE,
  RIGHT_BRACE,
  LEFT_BAR,
  RIGHT_BAR,
  COMMA,
  COLON,
  
  // variables
  STRING,
  BOOLEAN,
  NUMBER,
  NIL,

  // End of JSON
  EOJ

} TokenType;

#endif
