#include "Chaos.h"
#include "bindings.h"
#include <stdint.h>
#include <stdio.h>

void throw_err() {
  int err_len = last_error_length();
  char *err_str = malloc(err_len);
  last_error_message(err_str, err_len);
  kaos.raiseError(err_str);
  free(err_str);
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
  bool ret = is_match(re, text, is_match_opt_params[0].b);
  if (have_last_error()) {
    throw_err();
    return 1;
  }
  kaos.returnVariableBool(ret);
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
  if (have_last_error()) {
    throw_err();
    return 1;
  }
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
  if (have_last_error()) {
    throw_err();
    return 1;
  }
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
  if (have_last_error()) {
    throw_err();
    return 1;
  }
  kaos.returnVariableString(ret);
  free_str(ret);
  return 0;
}

char *split_params_name[] = {"re", "text", "cache"};
unsigned split_params_type[] = {K_STRING, K_STRING, K_BOOL};
unsigned split_params_secondary_type[] = {K_ANY, K_ANY, K_ANY};
unsigned short split_params_length =
    (unsigned short)sizeof(split_params_type) / sizeof(unsigned);
KaosValue split_opt_params[1];
int KAOS_EXPORT Kaos_split() {
  char *re = kaos.getVariableString(split_params_name[0]);
  char *text = kaos.getVariableString(split_params_name[1]);
  char **ret = NULL;
  uintptr_t len = split(re, text, split_opt_params[0].b, &ret);
  if (have_last_error()) {
    throw_err();
    return 1;
  }
  kaos.startBuildingList();
  for (uintptr_t i = 0; i < len; i++) {
    kaos.createVariableString(NULL, ret[i]);
  }
  kaos.returnList(K_STRING);
  free_str_list(ret, len);
  return 0;
}

char *splitn_params_name[] = {"re", "text", "limit", "cache"};
unsigned splitn_params_type[] = {K_STRING, K_STRING, K_NUMBER, K_BOOL};
unsigned splitn_params_secondary_type[] = {K_ANY, K_ANY, K_ANY, K_ANY};
unsigned short splitn_params_length =
    (unsigned short)sizeof(splitn_params_type) / sizeof(unsigned);
KaosValue splitn_opt_params[1];
int KAOS_EXPORT Kaos_splitn() {
  char *re = kaos.getVariableString(splitn_params_name[0]);
  char *text = kaos.getVariableString(splitn_params_name[1]);
  long long limit = kaos.getVariableInt(splitn_params_name[2]);
  char **ret = NULL;
  uintptr_t len = splitn(re, text, limit, splitn_opt_params[0].b, &ret);
  if (have_last_error()) {
    throw_err();
    return 1;
  }
  kaos.startBuildingList();
  for (uintptr_t i = 0; i < len; i++) {
    kaos.createVariableString(NULL, ret[i]);
  }
  kaos.returnList(K_STRING);
  free_str_list(ret, len);
  return 0;
}

char *find_params_name[] = {"re", "text", "cache"};
unsigned find_params_type[] = {K_STRING, K_STRING, K_BOOL};
unsigned find_params_secondary_type[] = {K_ANY, K_ANY, K_ANY};
unsigned short find_params_length =
    (unsigned short)sizeof(find_params_type) / sizeof(unsigned);
KaosValue find_opt_params[1];
int KAOS_EXPORT Kaos_find() {
  char *re = kaos.getVariableString(find_params_name[0]);
  char *text = kaos.getVariableString(find_params_name[1]);
  Match *ret = find(re, text, find_opt_params[0].b);
  if (have_last_error()) {
    throw_err();
    return 1;
  }
  kaos.startBuildingList();
  if (ret != NULL) {
    kaos.startBuildingDict();
    kaos.createVariableInt("start", ret->start);
    kaos.createVariableInt("end", ret->end);
    kaos.createVariableString("string", ret->string);
    kaos.finishDict(K_ANY);
  }
  kaos.returnList(K_DICT);
  free_match_prt(ret);
  return 0;
}

char *find_all_params_name[] = {"re", "text", "cache"};
unsigned find_all_params_type[] = {K_STRING, K_STRING, K_BOOL};
unsigned find_all_params_secondary_type[] = {K_ANY, K_ANY, K_ANY};
unsigned short find_all_params_length =
    (unsigned short)sizeof(find_all_params_type) / sizeof(unsigned);
KaosValue find_all_opt_params[1];
int KAOS_EXPORT Kaos_find_all() {
  char *re = kaos.getVariableString(find_all_params_name[0]);
  char *text = kaos.getVariableString(find_all_params_name[1]);
  Match *ret = NULL;
  uintptr_t len = find_all(re, text, find_all_opt_params[0].b, &ret);
  if (have_last_error()) {
    throw_err();
    return 1;
  }
  kaos.startBuildingList();
  for (uintptr_t i = 0; i < len; i++) {
    kaos.startBuildingDict();
    kaos.createVariableInt("start", ret[i].start);
    kaos.createVariableInt("end", ret[i].end);
    kaos.createVariableString("string", ret[i].string);
    kaos.finishDict(K_ANY);
  }
  kaos.returnList(K_DICT);
  free_match_list(ret, len);
  return 0;
}

// TODO Not useful until string paren bug is fixed
/* char *captures_params_name[] = {"re", "text", "cache"}; */
/* unsigned captures_params_type[] = {K_STRING, K_STRING, K_BOOL}; */
/* unsigned captures_params_secondary_type[] = {K_ANY, K_ANY, K_ANY}; */
/* unsigned short captures_params_length = */
/*     (unsigned short)sizeof(captures_params_type) / sizeof(unsigned); */
/* KaosValue captures_opt_params[1]; */
/* int KAOS_EXPORT Kaos_captures() { */
/*   char *re = kaos.getVariableString(captures_params_name[0]); */
/*   char *text = kaos.getVariableString(captures_params_name[1]); */
/*   MatchIdx *ret = NULL; */
/*   uintptr_t len = captures(re, text, captures_opt_params[0].b, &ret); */
/*   kaos.startBuildingDict(); */
/*   for (uintptr_t i = 0; i < len; i++) { */
/*     kaos.startBuildingDict(); */
/*     kaos.createVariableBool("test", true); */
/*     kaos.createVariableInt("test2", 77); */
/*     /1* kaos.createVariableInt("start", ret[i].start); *1/ */
/*     /1* kaos.createVariableInt("end", ret[i].end); *1/ */
/*     /1* kaos.createVariableString("string", ret[i].string); *1/ */
/*     kaos.finishDict(K_ANY); */
/*   } */
/*   kaos.returnDict(K_DICT); */
/*   /1* free_match_list(ret, len); *1/ */
/*   return 0; */
/* } */

int KAOS_EXPORT KaosRegister(struct Kaos _kaos) {
  kaos = _kaos;

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

  split_opt_params[0].b = true;
  kaos.defineFunction("split", K_STRING, K_ANY, split_params_name,
                      split_params_type, split_params_secondary_type,
                      split_params_length, split_opt_params, 1);

  splitn_opt_params[0].b = true;
  kaos.defineFunction("splitn", K_STRING, K_ANY, splitn_params_name,
                      splitn_params_type, splitn_params_secondary_type,
                      splitn_params_length, splitn_opt_params, 1);

  find_opt_params[0].b = true;
  kaos.defineFunction("find", K_STRING, K_ANY, find_params_name,
                      find_params_type, find_params_secondary_type,
                      find_params_length, find_opt_params, 1);

  find_all_opt_params[0].b = true;
  kaos.defineFunction("find_all", K_STRING, K_ANY, find_all_params_name,
                      find_all_params_type, find_all_params_secondary_type,
                      find_all_params_length, find_all_opt_params, 1);

  /* captures_opt_params[0].b = true; */
  /* kaos.defineFunction("captures", K_STRING, K_ANY, captures_params_name, */
  /*                     captures_params_type, captures_params_secondary_type,
   */
  /*                     captures_params_length, captures_opt_params, 1); */

  return 0;
}
