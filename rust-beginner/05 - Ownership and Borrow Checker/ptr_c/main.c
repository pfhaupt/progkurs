#include <stdio.h>

void somethingElse() {
    int a = 420;
}
int *f() {
    int x = 10;
    return &x;
}
int main(void) {
    int *hehe = f();
    printf("%d\n", *hehe);
    somethingElse();
    printf("%d\n", *hehe);
}