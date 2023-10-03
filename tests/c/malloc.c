#include <stdlib.h>

int main(void) {
  char *alloc = (char *)malloc(10);
  *alloc = 1;
  *(alloc + 9) = 2;
  free(alloc);
}