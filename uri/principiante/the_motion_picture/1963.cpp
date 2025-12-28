#include <iostream>
#include <iomanip>

int main() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);
    double a, b;
    std::cout << std::fixed << std::setprecision(2);
    if (std::cin >> a >> b) {
        double pct = (b - a) / a * 100.0;
        std::cout << pct << "%\n";
    }
    return 0;
}
