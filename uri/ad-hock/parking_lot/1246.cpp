#include <iostream>
#include <vector>
#include <unordered_map>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int c, n;
    while (cin >> c >> n) {
        vector<int> slots(c, 0);
        unordered_map<int, pair<int, int>> parked;
        int revenue = 0;
        for (int i = 0; i < n; ++i) {
            char type;
            cin >> type;
            if (type == 'C') {
                int plate, length;
                cin >> plate >> length;
                int start = -1;
                for (int pos = 0; pos + length <= c; ++pos) {
                    bool free = true;
                    for (int k = 0; k < length; ++k) {
                        if (slots[pos + k] != 0) {
                            free = false;
                            break;
                        }
                    }
                    if (free) {
                        start = pos;
                        break;
                    }
                }
                if (start != -1) {
                    for (int k = 0; k < length; ++k) {
                        slots[start + k] = plate;
                    }
                    parked[plate] = {start, length};
                    revenue += 10;
                }
            } else if (type == 'S') {
                int plate;
                cin >> plate;
                auto info = parked[plate];
                for (int k = 0; k < info.second; ++k) {
                    slots[info.first + k] = 0;
                }
                parked.erase(plate);
            }
        }
        cout << revenue << '\n';
    }

    return 0;
}
