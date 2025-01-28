#include <iostream>
using namespace std;

void foo() {
    cout << "Inside foo" << endl;
}

int main() {
    foo();
    cout << "In main" << endl;
    return 0;
}
