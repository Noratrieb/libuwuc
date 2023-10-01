#include<string.h>

int main(void) {
    char buf[10];

    memset(buf, 34, sizeof(buf));

    for (int i = 0; i < 10; ++i) {
        if (buf[i] != 34) {
            return 1;
        }
    }
}