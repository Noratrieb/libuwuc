#include <unistd.h>
#include <fcntl.h>
#include <assert.h>

int main(int argc, char *argv[])
{
    int alphabet = open("data/alphabet.txt", O_RDONLY);
    assert(alphabet != -1 && "failed to open file");

    off_t off;

    off = lseek(alphabet, 10, SEEK_SET);
    assert(off != -1 && "failed to seek file");

    char buf[1];
    ssize_t amount;

    amount = read(alphabet, buf, 1);
    assert(amount == 1 && "failed to read from alphabet");
    assert(buf[0] == 'k' && "character at offest 10 is not k");


    off = lseek(alphabet, 1, SEEK_CUR);
    assert(off != -1 && "failed to seek file");

    amount = read(alphabet, buf, 1);
    assert(amount == 1 && "failed to read from alphabet");
    assert(buf[0] == 'm' && "character at offest 12 is not m");
}
