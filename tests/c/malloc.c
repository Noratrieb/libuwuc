#include <stdlib.h>
#include <assert.h>

int main(void)
{
  char *alloc = (char *)malloc(10);
  assert(alloc && "allocation failed");
  *alloc = 1;
  *(alloc + 9) = 2;
  free(alloc);
}