// Sample for the NWCPP language panel in a bare bones language

// #include <threads.h>


#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

#define THREADS 100
#define INCS_PER_THREAD 10000

// Simple error reporting function
typedef enum { SUCCESS = 0, FAILURE = -1 } Result;

// Global counter and mutex for thread-safe operations
unsigned long long* COUNT = 0;
pthread_mutex_t count_mutex;

void* thread_function() {
    for (size_t i = 0; i < INCS_PER_THREAD; i++) {
        pthread_mutex_lock(&count_mutex);
        (*COUNT)++;
        pthread_mutex_unlock(&count_mutex);
    }
    return NULL;
}

int main() {
    printf("Spawning %d threads to increment `COUNT` %d times each...\n", THREADS, INCS_PER_THREAD);

    pthread_t threads[THREADS];

    // Allocate the memory for the counter.
    COUNT = (unsigned long long*)malloc(sizeof(unsigned long long));

    // Initialize the mutex
    if (pthread_mutex_init(&count_mutex, NULL) != 0) {
        perror("Mutex init failed");
        return FAILURE;
    }

    for (size_t i = 0; i < THREADS; i++) {
        if (pthread_create(&threads[i], NULL, thread_function, NULL) != 0) {
            perror("Thread creation failed");
            return FAILURE;
        }
    }

    for (size_t i = 0; i < THREADS; i++) {
        if (pthread_join(threads[i], NULL) != 0) {
            perror("Thread join failed");
            return FAILURE;
        }
    }

    // Destroy the mutex
    pthread_mutex_destroy(&count_mutex);

    printf("Expected total count: %d; Actual count: %llu\n", THREADS * INCS_PER_THREAD, *COUNT);

    // Delete the memory for the counter:
    free(COUNT);

    return SUCCESS;
}
