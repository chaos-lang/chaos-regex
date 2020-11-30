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

char *replace_params_name[] = {"re", "text", "rep", "cache"};
unsigned replace_params_type[] = {K_STRING, K_STRING, K_STRING, K_BOOL};
unsigned replace_params_secondary_type[] = {K_ANY, K_ANY, K_ANY, K_ANY};
unsigned short replace_params_length =
    (unsigned short)sizeof(replace_params_type) / sizeof(unsigned);
KaosValue replace_opt_params[1];
int KAOS_EXPORT Kaos_replace() {
  char *re = kaos.getVariableString(replace_params_name[0]);
  char *text = kaos.getVariableString(replace_params_name[1]);
  char *rep = kaos.getVariableString(replace_params_name[2]);
  char *ret = replace(re, text, rep, replace_opt_params[0].b);
  kaos.returnVariableString(ret);
  free_str(ret);
  return 0;
}

char *replace_all_params_name[] = {"re", "text", "rep", "cache"};
unsigned replace_all_params_type[] = {K_STRING, K_STRING, K_STRING, K_BOOL};
unsigned replace_all_params_secondary_type[] = {K_ANY, K_ANY, K_ANY, K_ANY};
unsigned short replace_all_params_length =
    (unsigned short)sizeof(replace_all_params_type) / sizeof(unsigned);
KaosValue replace_all_opt_params[1];
int KAOS_EXPORT Kaos_replace_all() {
  char *re = kaos.getVariableString(replace_all_params_name[0]);
  char *text = kaos.getVariableString(replace_all_params_name[1]);
  char *rep = kaos.getVariableString(replace_all_params_name[2]);
  char *ret = replace_all(re, text, rep, replace_all_opt_params[0].b);
  kaos.returnVariableString(ret);
  free_str(ret);
  return 0;
}

char *replacen_params_name[] = {"re", "text", "limit", "rep", "cache"};
unsigned replacen_params_type[] = {K_STRING, K_STRING, K_NUMBER, K_STRING,
                                   K_BOOL};
unsigned replacen_params_secondary_type[] = {K_ANY, K_ANY, K_ANY, K_ANY, K_ANY};
unsigned short replacen_params_length =
    (unsigned short)sizeof(replacen_params_type) / sizeof(unsigned);
KaosValue replacen_opt_params[1];
int KAOS_EXPORT Kaos_replacen() {
  char *re = kaos.getVariableString(replacen_params_name[0]);
  char *text = kaos.getVariableString(replacen_params_name[1]);
  long long limit = kaos.getVariableInt(replacen_params_name[2]);
  char *rep = kaos.getVariableString(replacen_params_name[3]);
  char *ret = replacen(re, text, limit, rep, replacen_opt_params[0].b);
  kaos.returnVariableString(ret);
  free_str(ret);
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

  replace_opt_params[0].b = true;
  kaos.defineFunction("replace", K_STRING, K_ANY, replace_params_name,
                      replace_params_type, replace_params_secondary_type,
                      replace_params_length, replace_opt_params, 1);

  replace_all_opt_params[0].b = true;
  kaos.defineFunction("replace_all", K_STRING, K_ANY, replace_all_params_name,
                      replace_all_params_type,
                      replace_all_params_secondary_type,
                      replace_all_params_length, replace_all_opt_params, 1);

  replacen_opt_params[0].b = true;
  kaos.defineFunction("replacen", K_STRING, K_ANY, replacen_params_name,
                      replacen_params_type, replacen_params_secondary_type,
                      replacen_params_length, replacen_opt_params, 1);
  return 0;
}
