#include<stdlib.h>

int main(void) {
    char *str = "12";
    long value = strtol(str, NULL, 10);
    if (value != 12) {
        return 1;
    }
}
