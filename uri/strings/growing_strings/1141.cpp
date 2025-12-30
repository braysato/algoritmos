#include <iostream>
#include <vector>
#include <string>
#include <algorithm>

using namespace std;

int main() {
    int n;
    
    while (cin >> n && n != 0) {
        vector<string> strs(n);
        
        for (int i = 0; i < n; i++) {
            cin >> strs[i];
        }
        
        sort(strs.begin(), strs.end(), [](const string& a, const string& b) {
            return a.length() < b.length();
        });
        
        vector<int> dp(n, 1);
        int result = 1;
        
        for (int i = 1; i < n; i++) {
            for (int j = i - 1; j >= 0; j--) {
                if (strs[i].length() - strs[j].length() > 1000) break;
                if (strs[i].find(strs[j]) != string::npos) {
                    dp[i] = max(dp[i], dp[j] + 1);
                }
            }
            result = max(result, dp[i]);
        }
        
        cout << result << endl;
    }
    
    return 0;
}
