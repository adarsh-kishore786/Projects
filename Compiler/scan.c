static int next(void) {
  int c;

  if (Putback) {
    c = Putback;
    Putback = 0;
    return c;
  }

  c = fgetc(InputFile);
  if ('\n' == c)
    Line++;
  return c;
}

static void putback(int c) {
  Putback = c;
}

static int skip(void) {
  int c;

  c = next();
  while (' ' == c || '\t' == c || '\n' == c || '\r' == c || '\f' == c)
    c = next();

  return c;
}

// Scan and return the next token found in the input.
// Return 1 if token is valid, and zero if no tokens left.
int scan(struct token* t) {
  int c = skip();

  switch(c) {
    case EOF:
      return 0;
    case '+':
      t->token = T_PLUS;
      break;
    case '-':
      t->token = T_MINUS;
      break;
    case '*':
      t->token = T_STAR;
      break;
    case '/':
      t->token = T_SLASH;
      break;
    default:
      // Have to deal with digits also
      if (isdigit(c)) {
        t->intvalue = scanint(c);
        t->token = T_INTLIT;
        break;
      }

      printf("Unrecognized character %c on line %d\n", c, Line);
      exit(1);
  }
  return 1;
}

static int scanint(c) {
  int k, val = 0;

  while ((k = chrpos("0123456789", c) >= 0)) {
    val = val * 10 + k;
    c = next();
  }

  // We hit a non-numeric character
  putback(c);
  return val;
}

static int chrpos(char *s, int c) {
  char *p = strchr(s, c);
  return (p ? p - s : -1);
}
