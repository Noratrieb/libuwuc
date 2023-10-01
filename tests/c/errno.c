//@ignore doens't initialize fs yet
#include<errno.h>

int main(void) {
    int err = errno;
    return err;
}