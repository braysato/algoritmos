#include <iostream>
#include <string>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string s;
    while (getline(cin, s)) {
        string part;
        if (!getline(cin, part)) {
            cout << "null value" << '\n';
            break;
        }
        if (part.empty()) {
            cout << "null value" << '\n';
            continue;
        }
        string result;
        result.reserve(s.size());
        for (char c : s) {
            result.push_back(c);
            if (result.size() >= part.size()) {
                bool match = true;
                for (size_t i = 0; i < part.size(); ++i) {
                    if (result[result.size() - part.size() + i] != part[i]) {
                        match = false;
                        break;
                    }
                }
                if (match) {
                    result.resize(result.size() - part.size());
                }
            }
        }
        if (result.empty()) {
            cout << "null value" << '\n';
        } else {
            cout << result << '\n';
        }
    }

    return 0;
}
