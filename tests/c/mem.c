#include <string.h>
#include <assert.h>

int main(void)
{
  char buf[10];

  memset(buf, 34, sizeof(buf));

  for (int i = 0; i < 10; ++i)
  {
    assert((buf[i] == 34) && "memset failed to write byte");
  }
}