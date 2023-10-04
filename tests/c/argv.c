#include <stdio.h>
#include <assert.h>

int main(int argc, char *argv[])
{
  char *self = argv[0];
  char first = self[0];

  assert((first < 128) && "argv[0] is not ascii/utf-8");
}