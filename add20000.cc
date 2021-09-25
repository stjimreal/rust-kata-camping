/*
 * @Date: 2021-09-26 02:40:21
 * @LastEditors: LIULIJING
 * @LastEditTime: 2021-09-26 03:30:14
 */
#include "pthread.h"
#include "stdio.h"
#include "errno.h"
#include "string.h"
#include "sys/time.h"
#include "stdlib.h"

inline void ERROR_CHECK(int err, const char *msg) {
    if (err == 0) {
        return;
    }
    perror(msg);
    printf(" strerror:%s\nreturn %d:%s\n", strerror(errno), err, strerror(err));
}

constexpr int N = 20000;
int kThreadPool = 8;

struct thread_resource
{
    int val;
    pthread_mutex_t mutex;
};


void* thread_func(void *ptr) {
    auto &resource = *(thread_resource*)ptr;
    for (int i = 0; i < N; ++i) {
        pthread_mutex_lock(&resource.mutex);
        resource.val += 1;
        pthread_mutex_unlock(&resource.mutex);
    }
    return NULL;
}

int main(int argc, char const *argv[])
{
    if (argc == 2) {
        kThreadPool = atoi(argv[1]);
        if (kThreadPool == 0) {
            printf("Usage: ./exec [num-of-threads]\n");
            return 0;
        }
    }
    thread_resource rs;
    struct timeval start, end;
    printf("val_address: %p\n", &rs.val);
    int res = pthread_mutex_init(&rs.mutex, NULL);
    ERROR_CHECK(res, "unable to create mutex");
    rs.val = 0;
    pthread_t id[kThreadPool];
    gettimeofday(&start, NULL);
    for (int i = 0; i < kThreadPool; ++i) {
        int res = pthread_create(&id[i], NULL, thread_func, &rs);
        ERROR_CHECK(res, "unable to create thread");
    }
    for (int i = 0; i < kThreadPool; ++i) {
        pthread_join(id[i], NULL);
        ERROR_CHECK(res, "unable to join thread");
    }
    gettimeofday(&end,NULL);
    printf("I am main thread, val = %d, val_address = %p, use time = %ld us.\n",rs.val,&rs.val,(end.tv_sec-start.tv_sec)*1000000+end.tv_usec-start.tv_usec);
    return 0;
}
