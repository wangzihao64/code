#include <iostream>
#include "pthread.h"
#include <vector>
#include <queue>
using namespace std;
const int NUM=4;
#define SIZE 1024
pthread_mutex_t my_lock = PTHREAD_MUTEX_INITIALIZER;
bool close=false;
class Thread;
queue<void(*)(Thread*)> q;
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
class ThreadPool
{
public:
    using func_t = function<void*(void*)>;
    void start()
    {
        for(const auto& t : _threads)
        {
            t->start(handldFunc,t);
        }
    }
    void join()
    {
        for(const auto& t : _threads)
        {
            t->join();
        }
    }
    static ThreadPool* GetInstance()
    {
        if(_tp== nullptr)
        {
            _tp=new ThreadPool();
        }
        return _tp;
    }
private:
    static void* handldFunc(void* args)
    {
        Thread* td=(Thread*) args;
        while(1) {
            pthread_mutex_lock(&my_lock);
            if(close&&q.empty()){
                pthread_mutex_unlock(&my_lock);
                break;
            }
            if (q.empty()) {
                pthread_mutex_unlock(&my_lock);
                continue;
            }
            function<void(Thread*)> func = q.front();
            q.pop();
            pthread_mutex_unlock(&my_lock);
            func(td);
        }
        return 0;
    }
    ThreadPool(int n=NUM)
    {
        for(int i=0;i<n;++i)
        {
            _threads.emplace_back(new Thread());
        }
    }
    vector<Thread*> _threads;
    func_t _func;
    static ThreadPool* _tp;
};
ThreadPool* ThreadPool::_tp=nullptr;
void task(void (*callback)(Thread*)){

    //派发任务
    q.push(callback);
}
void Print1(Thread* td){
    printf("%s正在执行:",td->_name.c_str());
    printf("画画\n");
}
void Print2(Thread* td){
    printf("%s正在执行:",td->_name.c_str());
    printf("睡觉\n");
}


int main() {
    ThreadPool::GetInstance()->start();
    task(Print1);
    task(Print2);
    task(Print2);
    task(Print1);
    task(Print2);
    close=true;
    ThreadPool::GetInstance()->join();



    return 0;
}
