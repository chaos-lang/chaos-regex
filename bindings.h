#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

bool is_match(char *re, char *text, bool cache);

char *replace(char *re, char *text, char *rep, bool cache);

char *replace_all(char *re, char *text, char *rep, bool cache);

char *replacen(char *re, char *text, int64_t limit, char *rep, bool cache);
