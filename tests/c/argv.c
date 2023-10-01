#include<stdio.h>

int main(int argc, char* argv[]) {
    char *self = argv[0];
    char first = self[0];
    if (first != '/') {
        return 1;
    }
}