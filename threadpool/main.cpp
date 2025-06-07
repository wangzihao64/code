#include <iostream>
#include "pthread.h"
#include <vector>
#include <queue>
using namespace std;
pthread_mutex_t my_lock = PTHREAD_MUTEX_INITIALIZER;
#define NUM 4
bool close=false;
queue<void(*)(char*)> q;
void task(void (*callback)(char*)){

    //派发任务
    q.push(callback);
}
void Print1(char* name){
    cout << "画画" << *name;
}
void Print2(char* name){
    cout << "吃饭" << *name;
}
void* handldFunc(void* args){
    char* name=(char*)args;
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
        char buffer[100];
        function<void(char*)> func = q.front();
        q.pop();
        pthread_mutex_unlock(&my_lock);
        func(name);

    }
}
int main() {
    vector<pthread_t> tids;
    for(int i=0;i<NUM;++i)
    {
        pthread_t tid;
        char* buffer=new char[100];
        int* args=new int(i);
        snprintf(buffer,sizeof buffer,"%d\n",*args);
        pthread_create(&tid, nullptr,handldFunc, buffer);
        tids.emplace_back(tid);
    }
    task(Print1);
    task(Print2);


    close=true;
    for(auto tid : tids){
        pthread_join(tid, nullptr);
    }

    return 0;
}
