#include <iostream>
#include <vector>
#include <set>
#include <cmath>

using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    
    int N, B;
    
    while (cin >> N >> B && (N != 0 || B != 0)) {
        vector<int> balls(B);
        
        for (int i = 0; i < B; i++) {
            cin >> balls[i];
        }
        
        set<int> possible_differences;
        
        for (int i = 0; i < B; i++) {
            for (int j = 0; j < B; j++) {
                int diff = abs(balls[i] - balls[j]);
                possible_differences.insert(diff);
            }
        }
        
        bool can_generate_all = true;
        for (int i = 0; i <= N; i++) {
            if (possible_differences.find(i) == possible_differences.end()) {
                can_generate_all = false;
                break;
            }
        }
        
        if (can_generate_all) {
            cout << "Y\n";
        } else {
            cout << "N\n";
        }
    }
    
    return 0;
}