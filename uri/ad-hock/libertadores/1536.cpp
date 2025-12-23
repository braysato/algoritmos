#include <iostream>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int t;
    if (!(cin >> t)) {
        return 0;
    }
    while (t--) {
        int m1, v1, m2, v2;
        char sep;
        cin >> m1 >> sep >> v1;
        cin >> m2 >> sep >> v2;
        int points1 = 0, points2 = 0;
        if (m1 > v1) {
            points1 += 3;
        } else if (m1 < v1) {
            points2 += 3;
        } else {
            points1 += 1;
            points2 += 1;
        }
        if (m2 > v2) {
            points2 += 3;
        } else if (m2 < v2) {
            points1 += 3;
        } else {
            points1 += 1;
            points2 += 1;
        }
        int goals1 = m1 + v2;
        int goals2 = v1 + m2;
        int diff1 = goals1 - goals2;
        int diff2 = -diff1;
        int away1 = v2;
        int away2 = v1;
        if (points1 > points2) {
            cout << "Time 1\n";
        } else if (points2 > points1) {
            cout << "Time 2\n";
        } else if (diff1 > diff2) {
            cout << "Time 1\n";
        } else if (diff2 > diff1) {
            cout << "Time 2\n";
        } else if (away1 > away2) {
            cout << "Time 1\n";
        } else if (away2 > away1) {
            cout << "Time 2\n";
        } else {
            cout << "Penaltis\n";
        }
    }
    return 0;
}
