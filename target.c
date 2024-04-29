#include <stdio.h>

int main() {
    char buf[100];
    printf("Enter input: ");
    fflush(stdout);
    if (fgets(buf, sizeof(buf), stdin) == NULL) {
        return 1; // Error reading input
    }
    printf("You entered %s\n", buf);
    return 0;
}
