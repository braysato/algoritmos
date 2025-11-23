#include <iostream>
#include <vector>
#include <string>
#include <algorithm>

using namespace std;

int main() {
    int n, m;
    
    while (cin >> n >> m && (n || m)) {
        cin.ignore();
        
        vector<string> emoticons(n);
        for (int i = 0; i < n; i++) {
            getline(cin, emoticons[i]);
        }
        
        int totalChanges = 0;
        
        for (int i = 0; i < m; i++) {
            string line;
            getline(cin, line);
            
            vector<pair<int, int>> occurrences;
            
            for (const string& emoticon : emoticons) {
                size_t pos = 0;
                while ((pos = line.find(emoticon, pos)) != string::npos) {
                    occurrences.push_back({pos, pos + emoticon.length() - 1});
                    pos++;
                }
            }
            
            if (occurrences.empty()) continue;
            
            sort(occurrences.begin(), occurrences.end(), [](const pair<int,int>& a, const pair<int,int>& b) {
                return a.second < b.second;
            });
            
            int lastHit = -1;
            for (const auto& occ : occurrences) {
                if (lastHit < occ.first) {
                    lastHit = occ.second;
                    totalChanges++;
                }
            }
        }
        
        cout << totalChanges << endl;
    }
    
    return 0;
}
