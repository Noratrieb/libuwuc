#include <stdlib.h>
#include <assert.h>

int main(int argc, char *argv[])
{
  char *env = getenv("PATH");
  assert(env && "PATH doesnt exist");

  char *env2 = getenv(
      "__some absolutely NONSENSE that no one would ever define please..");
  assert(!env2 && "nonsense environment variable found");
}
