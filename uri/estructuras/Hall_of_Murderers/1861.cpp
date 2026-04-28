#include <iostream>
#include <map>
#include <set>
#include <string>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    map<string, int> kills;
    set<string> dead;

    string killer, victim;
    while (cin >> killer >> victim) {
        kills[killer]++;
        dead.insert(victim);
    }

    cout << "HALL OF MURDERERS\n";
    for (const auto &entry : kills) {
        if (dead.find(entry.first) == dead.end()) {
            cout << entry.first << ' ' << entry.second << '\n';
        }
    }

    return 0;
}
