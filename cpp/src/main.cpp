#include "headers/helper.hpp"

int main(int argc, char* argv[]){
    if (1 == argc){         
        std::cout << "Please enter the path/to/the/rom" << std::endl;
    } 

    else { // ignore all argv[:2]
        try{
            const auto _data = read_file_helper(argv[1]);
        } catch(const std::exception& e){
            std::cerr << "Error: " << e.what() << std::endl;
            return 1;
        }
    }

    // std::cout << "Hello, World!" << std::endl; -- Test 1
    return 0;
}