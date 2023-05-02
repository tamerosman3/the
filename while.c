#include <stdlib.h>
#include <unistd.h>

int main() {
    size_t size = 1;
    while (1) {
        // Allocate memory
        void* ptr = malloc(size);
        if (ptr == NULL) {
            // Allocation failed, handle error
            exit(1);
        }

        // Fill memory to force the system to actually allocate it
        char* p = ptr;
        for (size_t i = 0; i < size; i++) {
            p[i] = 0;
        }

        // Sleep to slow down the loop
        sleep(1);

        // Increase the size of the allocation
        size *= 2;
    }

    return 0;
}
