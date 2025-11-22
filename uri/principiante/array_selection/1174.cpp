#include <iostream>
#include <iomanip>

using namespace std;

int main() {
    for (int i = 0; i < 100; i++) {
        double num;
        cin >> num;
        
        if (num <= 10.0) {
            cout << "A[" << i << "] = " << fixed << setprecision(1) << num << endl;
        }
    }
    
    return 0;
}
