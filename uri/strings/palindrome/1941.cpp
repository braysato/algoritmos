#include <iostream>
#include <string>
#include <vector>
#include <limits>
#include <sstream>

using namespace std;

struct Node {
    int special;
    int length;
};

inline bool better(const Node &a, const Node &b) {
    if (a.special != b.special) {
        return a.special > b.special;
    }
    return a.length > b.length;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string s;
    if (!(cin >> s)) {
        return 0;
    }
    int n = static_cast<int>(s.size());

    vector<bool> special(n, false);
    string rest;
    getline(cin, rest); 
    string line;
    if (!getline(cin, line)) {
        line.clear();
    }
    if (!line.empty()) {
        stringstream ss(line);
        int count = 0;
        ss >> count;
        for (int i = 0; i < count; ++i) {
            int idx;
            ss >> idx;
            if (idx >= 1 && idx <= n) {
                special[idx - 1] = true;
            }
        }
    }

    vector<vector<Node>> dp(n, vector<Node>(n, {0, 0}));

    for (int i = 0; i < n; ++i) {
        dp[i][i] = {special[i] ? 1 : 0, 1};
    }

    for (int len = 2; len <= n; ++len) {
        for (int i = 0; i + len - 1 < n; ++i) {
            int j = i + len - 1;
            Node best = dp[i + 1][j];
            if (better(dp[i][j - 1], best)) {
                best = dp[i][j - 1];
            }
            if (s[i] == s[j]) {
                Node candidate;
                int extra = (special[i] ? 1 : 0) + (special[j] ? 1 : 0);
                if (len == 2) {
                    candidate = {extra, 2};
                } else {
                    candidate.special = dp[i + 1][j - 1].special + extra;
                    candidate.length = dp[i + 1][j - 1].length + 2;
                }
                if (better(candidate, best)) {
                    best = candidate;
                }
            }
            dp[i][j] = best;
        }
    }

    cout << dp[0][n - 1].length << '\n';
    return 0;
}
