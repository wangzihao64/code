#include <iostream>
#include "pthread.h"
#include <vector>
#include <queue>
using namespace std;
const int NUM=4;
pthread_mutex_t my_lock = PTHREAD_MUTEX_INITIALIZER;
bool close=false;
queue<void(*)(int*)> q;
class Thread
{
public:
    Thread(void* (*HandleFunc)(void*),int n=NUM)
            :ids(n)
    {
        for(int i=0;i<n;i++)
        {
            ids[i]=i;
            pthread_t tid;
            pthread_create(&tid, nullptr,HandleFunc, &ids[i]);
            tids.emplace_back(tid);
        }
    }
    void ThreadJoin()
    {
        for(auto tid : tids){
            pthread_join(tid, nullptr);
        }
    }
    vector<pthread_t> tids;
    vector<int> ids;
};
void task(void (*callback)(int*)){

    //派发任务
    q.push(callback);
}
void Print1(int* name){
    printf("线程%d正在执行任务：画画\n",*name);
}
void Print2(int* name){
    printf("线程%d正在执行任务：睡觉\n",*name);
}
void* handldFunc(void* args){
    int* name=(int*)args;
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
        function<void(int*)> func = q.front();
        q.pop();
        pthread_mutex_unlock(&my_lock);
        func(name);
    }
    return 0;
}


int main() {
    Thread p1(handldFunc);
    task(Print1);
    task(Print2);
    task(Print2);
    task(Print1);
    task(Print2);


    close=true;
    p1.ThreadJoin();

    return 0;
}
