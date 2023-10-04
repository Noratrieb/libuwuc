#include <stdio.h>
#include <assert.h>

int main(int argc, char *argv[])
{
  int result = printf("Hello, world!\n");
  assert((result == 14) && "printf returned wrong number of chars");
}
