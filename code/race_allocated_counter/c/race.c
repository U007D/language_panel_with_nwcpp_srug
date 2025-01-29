// Sample for the NWCPP language panel in a bare bones language

// #include <threads.h>


#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

#define N_THREADS 100
#define INCS_PER_THREAD 10000

// Simple error reporting function
typedef enum { SUCCESS = 0, FAILURE = -1 } Result;

// Global counter and mutex for thread-safe operations
unsigned long long* COUNT = 0;
pthread_mutex_t count_mutex;

void* thread_function() {
    for (size_t i = 0; i < INCS_PER_THREAD; i++) {
        (*COUNT)++;
    }
    return NULL;
}

int main() {
    printf("Spawning %d threads to increment `COUNT` %d times each...\n", N_THREADS, INCS_PER_THREAD);

    pthread_t threads[N_THREADS];

    // Allocate the memory for the counter.
    COUNT = (unsigned long long*)malloc(sizeof(unsigned long long));

    for (size_t i = 0; i < N_THREADS; i++) {
        if (pthread_create(&threads[i], NULL, thread_function, NULL) != 0) {
            perror("Thread creation failed");
            return FAILURE;
        }
    }

    for (size_t i = 0; i < N_THREADS; i++) {
        if (pthread_join(threads[i], NULL) != 0) {
            perror("Thread join failed");
            return FAILURE;
        }
    }

    printf("Expected total count: %d; Actual count: %llu\n", N_THREADS * INCS_PER_THREAD, *COUNT);

    // Delete the memory for the counter:
    free(COUNT);

    return SUCCESS;
}
