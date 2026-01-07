#include <iostream>
#include <vector>
#include <string>
#include <algorithm>

using namespace std;

int main() {
    int n;
    
    while (cin >> n) {
        vector<string> codes(n);
        
        for (int i = 0; i < n; i++) {
            cin >> codes[i];
        }
        
        sort(codes.begin(), codes.end());
        
        for (const string& code : codes) {
            cout << code << endl;
        }
    }
    
    return 0;
}
