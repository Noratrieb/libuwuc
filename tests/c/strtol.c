#include <stdlib.h>
#include <assert.h>

int main(void)
{
  char *str = "12";
  long value = strtol(str, NULL, 10);
  assert((value == 12) && "value must be 12");
}
