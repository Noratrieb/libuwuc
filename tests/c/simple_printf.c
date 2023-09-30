#include<stdio.h>

int main(int argc, char *argv[]) {
    int result = printf("Hello, world!\n");
    if (result != 14) {
        return 1;
    }
}
