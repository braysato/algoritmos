#include <iostream>
#include <string>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string line;
    while (getline(cin, line)) {
        string result;
        result.reserve(line.size());
        for (char ch : line) {
            if ((ch == ',' || ch == '.') && !result.empty() && result.back() == ' ') {
                result.pop_back();
            }
            result.push_back(ch);
        }
        cout << result << '\n';
    }

    return 0;
}
