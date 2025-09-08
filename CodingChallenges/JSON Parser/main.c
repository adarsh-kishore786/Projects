#include <stdio.h>
#include <stdlib.h>

#include "exit.h"
#include "file.h"
#include "lexer.h"
#include "token.h"
#include "util.h"

int main(int argc, const char **argv) {
  if (argc < 2) 
    end_program("Usage: ./parse <file-name>", EXIT_NORMAL);

  const char* text = read_file(argv[1]);

  enum Token *tokens = get_tokens(text);
  if (tokens != NULL)
    print_tokens(&tokens);

  end_program("", EXIT_NORMAL);
}
