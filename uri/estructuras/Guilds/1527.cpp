#include <cstdio>
using namespace std;

const int MAXN = 100005;

int parent[MAXN];
long long points[MAXN];

int find(int x) {
    if (parent[x] != x) {
        parent[x] = find(parent[x]);
    }
    return parent[x];
}

void unite(int a, int b) {
    int pa = find(a);
    int pb = find(b);
    if (pa != pb) {
        parent[pb] = pa;
        points[pa] += points[pb];
    }
}

int main() {
    int n, m;
    while (scanf("%d %d", &n, &m) == 2 && (n || m)) {
        for (int i = 1; i <= n; i++) {
            parent[i] = i;
            scanf("%lld", &points[i]);
        }
        
        int wins = 0;
        for (int i = 0; i < m; i++) {
            int q, a, b;
            scanf("%d %d %d", &q, &a, &b);
            
            if (q == 1) {
                unite(a, b);
            } else {
                int pa = find(a);
                int pb = find(b);
                int rafael = find(1);
                
                if (pa == pb) continue;
                
                if (rafael == pa) {
                    if (points[pa] > points[pb]) wins++;
                } else if (rafael == pb) {
                    if (points[pb] > points[pa]) wins++;
                }
            }
        }
        
        printf("%d\n", wins);
    }
    
    return 0;
}
