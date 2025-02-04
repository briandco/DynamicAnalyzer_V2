#include <iostream>

#include <chrono>
#include <time.h>
#include <ctime>
#include <malloc.h>

#include <malloc.h>
using namespace std;


void foo() {
 
int start_var;

///////////////// Wall Time ////////////////////////
// Start measuring time
struct timespec begin_wall, end_wall; 
clock_gettime(CLOCK_REALTIME, &begin_wall);

///////////////// CPU Time /////////////////////////
// Start measuring time
struct timespec begin_CPU, end_CPU; 
clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &begin_CPU);
struct mallinfo mi_start = mallinfo();



    cout << "Inside foo" << endl;


    int * arr = new int [10];
    for (int i =0; i<10; i++)
    {
        arr[i] = i+1;
    }

    delete[] arr;
     

struct mallinfo mi_end = mallinfo();

// The field 'uordblks' contains the total allocated space in bytes.
std::cout << "Total heap allocated " << ((mi_end.uordblks - mi_start.uordblks)) << " bytes" << std::endl;

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
 
int start_var;

///////////////// Wall Time ////////////////////////
// Start measuring time
struct timespec begin_wall, end_wall; 
clock_gettime(CLOCK_REALTIME, &begin_wall);

///////////////// CPU Time /////////////////////////
// Start measuring time
struct timespec begin_CPU, end_CPU; 
clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &begin_CPU);
struct mallinfo mi_start = mallinfo();



    foo();//40
    foo();
    foo();
    cout << "In main" << endl;
struct mallinfo mi_end = mallinfo();

// The field 'uordblks' contains the total allocated space in bytes.
std::cout << "Total heap allocated " << ((mi_end.uordblks - mi_start.uordblks)) << " bytes" << std::endl;

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
