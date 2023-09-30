#include<stdio.h>
int main(int argc, char *argv[]) {
    int result = printf("Hello, world!\n");
    if (result != 15) {
        return 1;
    }
}
