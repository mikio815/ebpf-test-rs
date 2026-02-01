#include <iostream>
#include <thread>
#include <chrono>

int g_hp = 100;

void print_status(int l_hp) {
    std::cout << "LOCAL HP:" << l_hp << "  GLOBAL HP:" << g_hp << std::endl;
}

void take_damage(int *l_hp) {
    (*l_hp)--;
    g_hp--;
}

int main() {
    int hp = 100;

    std::cout << "game start" << std::endl;

    while (hp > 0) {
        print_status(hp);
        take_damage(&hp);
        std::this_thread::sleep_for(std::chrono::seconds(1));
    }

    std::cout << "game end" << std::endl;
}