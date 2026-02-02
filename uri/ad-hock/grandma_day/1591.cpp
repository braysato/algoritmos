#include <iostream>
#include <string>
#include <vector>
using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    
    int t;
    cin >> t;
    
    while (t--) {
        int l, c;
        cin >> l >> c;
        
        vector<string> grid(l);
        for (int i = 0; i < l; i++) {
            cin >> grid[i];
        }
        
        int p;
        cin >> p;
        
        while (p--) {
            string word;
            cin >> word;
            int len = word.length();
            int count = 0;
            
            if (len == 1) {
                for (int i = 0; i < l; i++) {
                    for (int j = 0; j < c; j++) {
                        if (grid[i][j] == word[0]) {
                            count++;
                        }
                    }
                }
            } else {
                for (int i = 0; i < l; i++) {
                    for (int j = 0; j <= c - len; j++) {
                        bool match = true;
                        for (int k = 0; k < len && match; k++) {
                            if (grid[i][j + k] != word[k]) match = false;
                        }
                        if (match) count++;
                    }
                }
                
                for (int j = 0; j < c; j++) {
                    for (int i = 0; i <= l - len; i++) {
                        bool match = true;
                        for (int k = 0; k < len && match; k++) {
                            if (grid[i + k][j] != word[k]) match = false;
                        }
                        if (match) count++;
                    }
                }
            }
            
            cout << count << '\n';
        }
    }
    
    return 0;
}
