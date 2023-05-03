#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>

void cpu_intensive_task() {
    unsigned long long counter = 0;
    for (unsigned long long i = 0; i < 1000000000; i++) {
        counter += i;
    }
}

int main() {
    int interval_seconds = 1;

    while (1) {
        time_t start_time = time(NULL);
        time_t end_time = start_time + interval_seconds;

        // Perform the CPU-intensive task
        cpu_intensive_task();

        // Sleep for the remaining time in the interval, if any
        time_t current_time = time(NULL);
        if (current_time < end_time) {
            sleep(end_time - current_time);
        }

        // Increase the interval length
        interval_seconds *= 2;
    }

    return 0;
}
