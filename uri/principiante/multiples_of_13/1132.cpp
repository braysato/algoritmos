#include <iostream>
#include <algorithm>

using namespace std;

int main() {
    int x, y;
    cin >> x >> y;
    
    int minVal = min(x, y);
    int maxVal = max(x, y);
    
    int sum = 0;
    
    for (int i = minVal; i <= maxVal; i++) {
        if (i % 13 != 0) {
            sum += i;
        }
    }
    
    cout << sum << endl;
    
    return 0;
}
