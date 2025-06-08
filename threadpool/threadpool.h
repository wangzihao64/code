//
// Created by 王子豪 on 2025/6/8.
//

#ifndef THREADPOOL_THREADPOOL_H
#define THREADPOOL_THREADPOOL_H
#include "thread.h"
const int NUM=4;
#endif //THREADPOOL_THREADPOOL_H
pthread_mutex_t my_lock = PTHREAD_MUTEX_INITIALIZER;
class ThreadPool;
class ThreadData
{
public:
    ThreadData(ThreadPool* pool,string name)
            :_pool(pool),
             _name(name)
    {}
    ThreadPool* _pool;
    string _name;
};
class ThreadPool
{
public:
    using func_t = function<void*(void*)>;
    void start()
    {
        for(const auto& t : _threads)
        {
            ThreadData* thdata=new ThreadData(this,t->_name);
            t->start(handldFunc, thdata);
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
    void Push(void (*callback)(void*))
    {
        //派发任务
        _q.push(callback);
    }
    void Close()
    {
        close= true;
        join();
    }
private:
    static void* handldFunc(void* args)
    {
        ThreadData* td=(ThreadData*) args;
        while(1) {
            pthread_mutex_lock(&my_lock);
            if(td->_pool->close&&td->_pool->_q.empty()){
                pthread_mutex_unlock(&my_lock);
                break;
            }
            if (td->_pool->_q.empty()) {
                pthread_mutex_unlock(&my_lock);
                continue;
            }
            function<void(ThreadData*)> func = td->_pool->_q.front();
            td->_pool->_q.pop();
            pthread_mutex_unlock(&my_lock);
            printf("%s处理了这个任务，并处理完成了",td->_name.c_str());
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
    //任务队列
    queue<void(*)(void*)> _q;
    static ThreadPool* _tp;
    bool close=false;
};
ThreadPool* ThreadPool::_tp=nullptr;
