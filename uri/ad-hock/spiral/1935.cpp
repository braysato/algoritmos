#include <iostream>

using namespace std;

int main() {
    long long N, B;
    cin >> N >> B;
    
    long long layer = 0;
    long long total = 0;
    
    while (true) {
        long long side = N - 2 * layer;
        if (side <= 0) break;
        
        long long perimeter;
        if (side == 1) {
            perimeter = 1;
        } else {
            perimeter = 4 * side - 4;
        }
        
        if (total + perimeter >= B) {
            break;
        }
        
        total += perimeter;
        layer++;
    }
    
    long long offset = B - total - 1;
    long long side = N - 2 * layer;
    long long row = 1 + layer;
    long long col = 1 + layer;
    
    if (side == 1) {
        cout << row << " " << col << endl;
        return 0;
    }
    
    if (offset < side - 1) {
        row = 1 + layer;
        col = 1 + layer + offset;
    } else if (offset < 2 * (side - 1)) {
        row = 1 + layer + (offset - (side - 1));
        col = N - layer;
    } else if (offset < 3 * (side - 1)) {
        row = N - layer;
        col = N - layer - (offset - 2 * (side - 1));
    } else {
        row = N - layer - (offset - 3 * (side - 1));
        col = 1 + layer;
    }
    
    cout << row << " " << col << endl;
    
    return 0;
}

