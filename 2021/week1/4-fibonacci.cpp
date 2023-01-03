#include <iostream>
#include <string>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

int fib(int n) {
    int t1 = 0;
    int t2 = 1;

    // should be i < n
    for (int i = 1; i <= n; i++) {
        int next = t1 + t2;
        t1 = t2;
        t2 = next;
    }

    // if n == 0, should return 0
    return t2;
}

int main() {
    while (true) {
        std::cout << "Enter a number (or ctrl+c to exit): " << std::flush;
        std::string input;
        getline(std::cin, input);

        int parsed;
        try {
            parsed = std::stoi(input);
        } catch (std::invalid_argument &e) {
            std::cout << "That's not a valid number." << std::endl;
            continue;
        }

        std::cout << "fib(" << parsed << ") = " << fib(parsed) << std::endl;
    }
}
