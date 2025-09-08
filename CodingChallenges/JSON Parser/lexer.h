#ifndef LEXER_H
#define LEXER_H

struct Token* get_tokens(const char*);
struct Token variable(const char*);
enum TokenType get_simple_token_type(char);
void print_tokens(struct Token**);
const char* get_string_token_type(struct Token*);

#endif
