#include "headers/helper.hpp"

std::vector <unsigned char> read_file_helper(char const* filepath){
    // Helper function implementation
    
    try{
        std::ifstream file(filepath, std::ios::binary);
        std::uintmax_t size = std::filesystem::file_size(filepath);
        std::vector<unsigned char> buffer(size);
        file.read(reinterpret_cast<char*>(buffer.data()), size);
        file.close();
        return buffer;
    }
    catch(const std::exception& e){
        std::cerr << "Error: " << e.what() << std::endl;
        return std::vector<unsigned char>{};
    }
}
