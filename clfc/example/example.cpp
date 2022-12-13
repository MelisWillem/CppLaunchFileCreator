#include <iostream>

int main(int argv, char *args[])
{
    std::cout << "Called example.cpp with " << argv - 1 << " arguments" << std::endl;

    for (int i = 1; i < argv; ++i)
    {
        std::cout << "args[" << i << "]=" << args[i] << std::endl;
    }

    return 0;
}