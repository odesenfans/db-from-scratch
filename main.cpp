#include <iostream>
#include <memory>
#include <string>

std::string read_input()
{
    std::string line;
    std::getline(std::cin, line);
    return line;
}

void print_prompt()
{
    std::cout << ("db > ");
}

int main(int, char*[])
{
    while (true) {
        print_prompt();
        const auto line = read_input();

        if (line == ".exit") {
            exit(EXIT_SUCCESS);
        }
        else {
            std::cout << "Unrecoginized command: '" << line << "'.\n";
        }
    }
}
