#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    while (true) {
        int a, b, c, x, y;
        if (!(cin >> a >> b >> c >> x >> y)) {
            break;
        }
        if (a == 0 && b == 0 && c == 0 && x == 0 && y == 0) {
            break;
        }
        vector<int> princess = {a, b, c};
        vector<int> prince = {x, y};
        vector<bool> used(53, false);
        for (int v : princess) {
            used[v] = true;
        }
        for (int v : prince) {
            used[v] = true;
        }
        sort(princess.begin(), princess.end());
        int answer = -1;
        for (int candidate = 1; candidate <= 52; ++candidate) {
            if (used[candidate]) {
                continue;
            }
            vector<int> order = prince;
            order.push_back(candidate);
            sort(order.begin(), order.end());
            int minWins = 3;
            do {
                vector<int> available = princess;
                int princeWins = 0;
                for (int card : order) {
                    auto it = upper_bound(available.begin(), available.end(), card);
                    if (it == available.end()) {
                        available.erase(available.begin());
                        ++princeWins;
                    } else {
                        available.erase(it);
                    }
                }
                minWins = min(minWins, princeWins);
            } while (next_permutation(order.begin(), order.end()));
            if (minWins >= 2) {
                answer = candidate;
                break;
            }
        }
        cout << answer << "\n";
    }

    return 0;
}
