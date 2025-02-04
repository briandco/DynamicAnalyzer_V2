#include "transporter.h"
#include <iostream>

Transporter::Transporter() {
    std::cout << "Transporter constructor called" <<std::endl;
}

void Transporter::display() {
    std::cout << "Transport display called" << std::endl;
}
