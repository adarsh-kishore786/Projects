#ifndef LEXER_H
#define LEXER_H

enum Token* get_tokens(const char*);
enum Token get_token(char);
void print_tokens(enum Token**);
const char* get_string_token(enum Token*);

#endif
