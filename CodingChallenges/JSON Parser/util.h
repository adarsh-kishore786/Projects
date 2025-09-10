#ifndef UTIL_H
#define UTIL_H

#include "token_type.h"

const char* get_string_token_type(enum TokenType*);
void end_program(const char*, int);
char* join_string(const char*, const char*);

#endif
