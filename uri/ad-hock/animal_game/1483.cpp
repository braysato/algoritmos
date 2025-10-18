#include <iostream>
#include <iomanip>

using namespace std;

int getGroup(int lastTwoDigits) {
    if (lastTwoDigits == 0) return 24;
    return (lastTwoDigits - 1) / 4;
}

int main() {
    double v;
    int n, m;
    
    while (cin >> v >> n >> m) {
        if (v == 0 && n == 0 && m == 0) break;
        
        double prize = 0.0;
        
        int last4N = n % 10000;
        int last4M = m % 10000;
        int last3N = n % 1000;
        int last3M = m % 1000;
        int last2N = n % 100;
        int last2M = m % 100;
        
        if (last4N == last4M) {
            prize = v * 3000;
        } else if (last3N == last3M) {
            prize = v * 500;
        } else if (last2N == last2M) {
            prize = v * 50;
        } else if (getGroup(last2N) == getGroup(last2M)) {
            prize = v * 16;
        }
        
        cout << fixed << setprecision(2) << prize << endl;
    }
    
    return 0;
}
