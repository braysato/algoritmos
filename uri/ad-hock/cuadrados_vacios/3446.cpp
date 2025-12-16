#include <iostream>
#include <vector>
#include <bitset>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n, k, e;
    if (!(cin >> n >> k >> e)) {
        return 0;
    }
    int left_len = e;
    int right_len = n - k - e;
    const int MAXR = 1000;
    vector<bitset<MAXR + 1>> dp(left_len + 1);
    vector<int> pieces;
    pieces.reserve(n - 1);
    for (int len = 1; len <= n; ++len) {
        if (len != k) {
            pieces.push_back(len);
        }
    }
    bitset<MAXR + 1> mask;
    for (int i = 0; i <= right_len; ++i) {
        mask.set(i);
    }
    dp[0].set(0);
    for (int w : pieces) {
        vector<bitset<MAXR + 1>> next = dp;
        if (w <= left_len) {
            for (int l = w; l <= left_len; ++l) {
                next[l] |= dp[l - w];
            }
        }
        if (w <= right_len) {
            for (int l = 0; l <= left_len; ++l) {
                bitset<MAXR + 1> shifted = dp[l] << w;
                shifted &= mask;
                next[l] |= shifted;
            }
        }
        dp.swap(next);
    }
    int best = 0;
    for (int l = 0; l <= left_len; ++l) {
        for (int r = right_len; r >= 0; --r) {
            if (dp[l].test(r)) {
                if (l + r > best) {
                    best = l + r;
                }
                break;
            }
        }
    }
    int result = (n - k) - best;
    cout << result << '\n';
    return 0;
}
