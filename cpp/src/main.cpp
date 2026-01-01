#include <iostream>

void call_func(const char* rom){
    std::cout << "Calling game init for: " << rom << std::endl;
}

int main(int argc, char* argv[]){
    
    if (1 == argc){
        std::cout << "Please enter the path/to/the/rom" << std::endl;
    } 

    else { // ignore all argv[:2]
        try{
            call_func(argv[1]);
        } catch(const std::exception& e){
            std::cerr << "Error: " << e.what() << std::endl;
            return 1;
        }
    }

    // std::cout << "Hello, World!" << std::endl; -- Test 1
    return 0;
}