// Currently our language will support the basic
// arithmetic operators and the digits 0 through 9
enum {
  T_PLUS, T_MINUS, T_STAR, T_SLASH, T_INTLIT
}

struct token {
  int token;
  int intvalue;
}
