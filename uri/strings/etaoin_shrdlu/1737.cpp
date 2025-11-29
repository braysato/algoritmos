#include <iostream>
#include <vector>
#include <string>
#include <unordered_map>
#include <algorithm>
#include <iomanip>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cout << fixed << setprecision(6);

    int n;
    while (cin >> n) {
        if (n == 0) {
            break;
        }
        string dummy;
        getline(cin, dummy);
        string text;
        text.reserve(static_cast<size_t>(n) * 80);
        for (int i = 0; i < n; ++i) {
            string line;
            getline(cin, line);
            text += line;
        }
        if (text.size() < 2) {
            cout << "\n";
            continue;
        }
        unordered_map<string, int> counts;
        counts.reserve(text.size());
        for (size_t i = 0; i + 1 < text.size(); ++i) {
            string key = text.substr(i, 2);
            counts[key] += 1;
        }
        vector<pair<string, int>> items;
        items.reserve(counts.size());
        for (const auto& entry : counts) {
            items.push_back(entry);
        }
        sort(items.begin(), items.end(), [](const auto& a, const auto& b) {
            if (a.second != b.second) {
                return a.second > b.second;
            }
            return a.first < b.first;
        });
        int total = static_cast<int>(text.size() - 1);
        size_t limit = min<size_t>(5, items.size());
        for (size_t i = 0; i < limit; ++i) {
            double relative = static_cast<double>(items[i].second) / total;
            cout << items[i].first << " " << items[i].second << " " << relative << "\n";
        }
        cout << "\n";
    }

    return 0;
}
