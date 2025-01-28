#include <iostream>
#include <malloc.h>
#include <chrono>
#include <time.h>
#include <ctime>
using namespace std;

void foo() {
struct mallinfo mi_start =  mallinfo();
int start_var;

///////////////// Wall Time ////////////////////////
// Start measuring time
struct timespec begin_wall, end_wall; 
clock_gettime(CLOCK_REALTIME, &begin_wall);

///////////////// CPU Time /////////////////////////
// Start measuring time
struct timespec begin_CPU, end_CPU; 
clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &begin_CPU);

    cout << "Inside foo" << endl;
int end_var;

struct mallinfo mi_end = mallinfo();
cout << "\nStack used = " << ((reinterpret_cast<intptr_t>(&end_var))-(reinterpret_cast<intptr_t>(&start_var)) - 4);
cout << "\nHeap used = " << (mi_end.uordblks - mi_start.uordblks) << " bytes \n";

//////////////// Wall Time ///////////////
clock_gettime(CLOCK_REALTIME, &end_wall);
long seconds = end_wall.tv_sec - begin_wall.tv_sec;
long nanoseconds = end_wall.tv_nsec - begin_wall.tv_nsec;

// Convert the elapsed time to microseconds
double elapsed = (seconds * 1e6) + (nanoseconds * 1e-3);

printf("Wall Time measured: %.3f microseconds.\n", elapsed);

//////////////// CPU Time ///////////////
clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &end_CPU);
long seconds_CPU = end_CPU.tv_sec - begin_CPU.tv_sec;
long nanoseconds_CPU = end_CPU.tv_nsec - begin_CPU.tv_nsec;

// Convert the elapsed time to microseconds
double elapsed_CPU = (seconds_CPU * 1e6) + (nanoseconds_CPU * 1e-3);

printf("CPU Time measured: %.3f microseconds.\n\n", elapsed_CPU);
}

int main() {
struct mallinfo mi_start =  mallinfo();
int start_var;

///////////////// Wall Time ////////////////////////
// Start measuring time
struct timespec begin_wall, end_wall; 
clock_gettime(CLOCK_REALTIME, &begin_wall);

///////////////// CPU Time /////////////////////////
// Start measuring time
struct timespec begin_CPU, end_CPU; 
clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &begin_CPU);

    foo();
    cout << "In main" << endl;
int end_var;

struct mallinfo mi_end = mallinfo();
cout << "\nStack used = " << ((reinterpret_cast<intptr_t>(&end_var))-(reinterpret_cast<intptr_t>(&start_var)) - 4);
cout << "\nHeap used = " << (mi_end.uordblks - mi_start.uordblks) << " bytes \n";

//////////////// Wall Time ///////////////
clock_gettime(CLOCK_REALTIME, &end_wall);
long seconds = end_wall.tv_sec - begin_wall.tv_sec;
long nanoseconds = end_wall.tv_nsec - begin_wall.tv_nsec;

// Convert the elapsed time to microseconds
double elapsed = (seconds * 1e6) + (nanoseconds * 1e-3);

printf("Wall Time measured: %.3f microseconds.\n", elapsed);

//////////////// CPU Time ///////////////
clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &end_CPU);
long seconds_CPU = end_CPU.tv_sec - begin_CPU.tv_sec;
long nanoseconds_CPU = end_CPU.tv_nsec - begin_CPU.tv_nsec;

// Convert the elapsed time to microseconds
double elapsed_CPU = (seconds_CPU * 1e6) + (nanoseconds_CPU * 1e-3);

printf("CPU Time measured: %.3f microseconds.\n\n", elapsed_CPU);
    return 0;
}
