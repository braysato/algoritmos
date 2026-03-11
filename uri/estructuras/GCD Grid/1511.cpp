#include <cstdio>
#include <cstring>
#include <algorithm>
#include <vector>
using namespace std;

long long grid[1001][1001];

inline long long gcd(long long a, long long b) {
    while (b) { a %= b; swap(a, b); }
    return a;
}

int main() {
    int Q;
    while (scanf("%d", &Q) == 1) {
        memset(grid, 0, sizeof(grid));
        
        vector<int> px, py;
        px.reserve(Q);
        py.reserve(Q);
        
        long long gcdGlobal = 0;
        bool gcdDirty = true;

        while (Q--) {
            char type[10];
            int x, y;
            long long d;
            scanf("%s %d %d %lld", type, &x, &y, &d);

            int gx = x + 500;
            int gy = y + 500;

            if (type[0] == 'S') {
                if (grid[gx][gy] == 0) {
                    px.push_back(x);
                    py.push_back(y);
                }
                grid[gx][gy] = d;
                gcdDirty = true;
            } else {
                long long result = 0;
                int n = px.size();
                
                if (d >= 2000) {
                    if (gcdDirty) {
                        gcdGlobal = 0;
                        for (int i = 0; i < n && gcdGlobal != 1; i++) {
                            gcdGlobal = gcd(gcdGlobal, grid[px[i] + 500][py[i] + 500]);
                        }
                        gcdDirty = false;
                    }
                    result = gcdGlobal;
                } else {
                    for (int i = 0; i < n && result != 1; i++) {
                        int dist = abs(px[i] - x) + abs(py[i] - y);
                        if (dist <= d) {
                            result = gcd(result, grid[px[i] + 500][py[i] + 500]);
                        }
                    }
                }
                printf("%lld\n", result);
            }
        }
    }

    return 0;
}
