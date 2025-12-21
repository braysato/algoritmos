#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int C;
    if (!(cin >> C)) {
        return 0;
    }
    while (C--) {
        int N;
        cin >> N;
        unordered_set<string> owned;
        for (int i = 0; i < N; ++i) {
            string item, cmd;
            cin >> item >> cmd;
            if (cmd == "chirrin") {
                if (!owned.count(item)) {
                    owned.insert(item);
                }
            } else if (cmd == "chirrion") {
                auto it = owned.find(item);
                if (it != owned.end()) {
                    owned.erase(it);
                }
            }
        }
        cout << "TOTAL\n";
        vector<string> sorted_items(owned.begin(), owned.end());
        sort(sorted_items.begin(), sorted_items.end());
        for (const auto& item : sorted_items) {
            cout << item << '\n';
        }
    }
    return 0;
}
