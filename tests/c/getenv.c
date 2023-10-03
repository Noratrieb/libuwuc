#include <stdlib.h>

int main(int argc, char *argv[]) {
  char *env = getenv("PATH");
  if (!env) {
    return 1;
  }

  char *env2 = getenv(
      "__some absolutely NONSENSE that no one would ever define please..");
  if (env2) {
    return 1;
  }
}
