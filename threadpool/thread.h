//
// Created by 王子豪 on 2025/6/8.
//

#ifndef THREADPOOL_THREAD_H
#define THREADPOOL_THREAD_H

#endif //THREADPOOL_THREAD_H
#include <iostream>
#include "pthread.h"
#include <vector>
#include <queue>
#define SIZE 1024
using namespace std;
class Thread
{
public:
    using func_t = function<void*(void*)>;
private:
    static void* start_routine(void* args)
    {
        Thread* ctx=(Thread*) args;
        return ctx->callback();
    }
public:
    Thread()
    {
        char buffer[SIZE];
        snprintf(buffer,sizeof buffer,"thread is %d",threadnum);
        threadnum++;
        _name=buffer;
    }
    void start(func_t func,void* args= nullptr){
        _func=func;
        _args=args;
        pthread_create(&_tid, nullptr,start_routine,this);
    }
    void* callback()
    {
        return _func(_args);
    }
    void join()
    {
        pthread_join(_tid, nullptr);
    }
    string _name;
    pthread_t _tid;
    func_t _func;
    void* _args;
    static int threadnum;
};
int Thread::threadnum=0;