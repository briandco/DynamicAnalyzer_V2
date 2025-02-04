#include <iostream>
#include <malloc.h>
using namespace std;


void foo() {
    cout << "Inside foo" << endl;


    int * arr = new int [10];
    for (int i =0; i<10; i++)
    {
        arr[i] = i+1;
    }

    delete[] arr;
     

}

int main() {
    foo();//40
    foo();
    foo();
    cout << "In main" << endl;
    return 0;
}
