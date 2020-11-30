#include "Chaos.h"
#include "bindings.h"

char *hello_params_name[] = {};
unsigned hello_params_type[] = {};
unsigned hello_params_secondary_type[] = {};
unsigned short hello_params_length = 0;
int KAOS_EXPORT Kaos_hello() {
  fprintf(stdout, "Hello from the template!\n");
  fflush(stdout);
  return 0;
}

char *is_match_params_name[] = {"re", "text", "cache"};
unsigned is_match_params_type[] = {K_STRING, K_STRING, K_BOOL};
unsigned is_match_params_secondary_type[] = {K_ANY, K_ANY, K_ANY};
unsigned short is_match_params_length =
    (unsigned short)sizeof(is_match_params_type) / sizeof(unsigned);
KaosValue is_match_opt_params[1];
int KAOS_EXPORT Kaos_is_match() {
  char *re = kaos.getVariableString(is_match_params_name[0]);
  char *text = kaos.getVariableString(is_match_params_name[1]);
  kaos.returnVariableBool(is_match(re, text, is_match_opt_params[0].b));
  return 0;
}

int KAOS_EXPORT KaosRegister(struct Kaos _kaos) {
  kaos = _kaos;
  kaos.defineFunction("hello", K_VOID, K_ANY, hello_params_name,
                      hello_params_type, hello_params_secondary_type,
                      hello_params_length, NULL, 0);

  is_match_opt_params[0].b = true;
  kaos.defineFunction("is_match", K_BOOL, K_ANY, is_match_params_name,
                      is_match_params_type, is_match_params_secondary_type,
                      is_match_params_length, is_match_opt_params, 1);

  return 0;
}
