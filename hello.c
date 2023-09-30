#include<stdio.h>
#include<sys/mman.h>
int main(int argc, char *argv[]) {
    PROT_WRITE;
    puts("Hello, world!");
}
