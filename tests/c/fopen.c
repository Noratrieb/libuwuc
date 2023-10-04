#include <stdio.h>
#include <errno.h>
#include <assert.h>

int main(int argc, char *argv[]) {
    FILE *file1 = fopen("./c/fopen.c", "r");
    assert(file1 && "failed to open file");
    
    FILE *file2 = fopen("./c/fopen.c", "meow");
    assert(!file2 && "succeeded opening file despite invalid argument");
    assert((errno == EINVAL) && "wrong errno");

    FILE *file3 = fopen("/this-does-absolutely-not-exist-at-all-edhjkefhew98", "r");
    assert(!file3 && "succeeded despite file not existing");
    assert((errno == ENOENT) && "wrong errno");
}