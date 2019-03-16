#include <stdio.h>
#include <stdlib.h>

int main() {
    int lim = rand() % 100 + 5 * 1e8;
    for (int i = 0; i < lim; ++i) {
        if (i % 100000000 == 0) {
            printf("%d ", i);
        }
    }
    printf("\n");
    return 0;
}