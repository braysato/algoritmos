#include <iostream>
#include <vector>
#include <string>
#include <algorithm>

using namespace std;

int main() {
    int n;
    bool first = true;
    
    while (cin >> n && n != 0) {
        if (!first) {
            cout << endl;
        }
        first = false;
        
        vector<string> words(n);
        int maxLength = 0;
        
        for (int i = 0; i < n; i++) {
            cin >> words[i];
            maxLength = max(maxLength, (int)words[i].length());
        }
        
        for (int i = 0; i < n; i++) {
            int spaces = maxLength - words[i].length();
            for (int j = 0; j < spaces; j++) {
                cout << " ";
            }
            cout << words[i] << endl;
        }
    }
    
    return 0;
}
