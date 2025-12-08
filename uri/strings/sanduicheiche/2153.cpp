#include <iostream>
#include <string>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string word;
    while (cin >> word) {
        int n = static_cast<int>(word.size());
        string corrected = word;
        for (int len = 1; len * 2 <= n; ++len) {
            if (word.substr(n - 2 * len, len) == word.substr(n - len, len)) {
                corrected = word.substr(0, n - len);
                break;
            }
        }
        cout << corrected << '\n';
    }

    return 0;
}
