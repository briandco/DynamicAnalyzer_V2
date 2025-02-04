#include <iostream>
#include "transporter.h"
#include "vehicle.h"

int main() {
    Transporter TransObj;
    Vehicle Hyundai;

    TransObj.display();
    Hyundai.display();

    std::cout << "Main function execution complete!" << std::endl;

    return 0;
}
