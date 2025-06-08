#include "threadpool.h"



void Print1(void* args){
    printf("画画\n");
}
void Print2(void* args){
    printf("睡觉\n");
}


int main() {
    ThreadPool::GetInstance()->start();
    ThreadPool::GetInstance()->Push(Print1);
    ThreadPool::GetInstance()->Push(Print1);
    ThreadPool::GetInstance()->Push(Print2);
    ThreadPool::GetInstance()->Push(Print2);
    ThreadPool::GetInstance()->Push(Print1);
    ThreadPool::GetInstance()->Push(Print2);
    ThreadPool::GetInstance()->Push(Print2);
    ThreadPool::GetInstance()->Close();

    return 0;
}
