#include "vehicle.h"
#include <iostream>

Vehicle::Vehicle(){
    std::cout << "Vehicle Initialised" << std::endl;
}

void Vehicle::start(){
    std::cout << "Vehicle started!" << std::endl;
}

void Vehicle::stop(){
    std::cout << "Vehicle halted!" << std::endl;
}

void Vehicle::speedUp(int speed){
    std::cout << "Vehicle speedin up by 10%" << std::endl;
}

void Vehicle::display() {
    std::cout << "Hello from Class B!" << std::endl;
}