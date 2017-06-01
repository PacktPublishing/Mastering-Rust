#include <stdio.h>

int* f1() {
    int x = 4;
    return &x;
}

int main() {
    int *f1s_x = f1();
    printf("f1 returned %d", *f1s_x);
}
