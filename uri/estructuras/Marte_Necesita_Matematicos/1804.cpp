#include <cstdio>
#include <cstring>

const int MAXN = 100005;

int bit[MAXN], p[MAXN], n;

void update(int i, int val) {
    for (; i <= n; i += i & (-i))
        bit[i] += val;
}

int query(int i) {
    int s = 0;
    for (; i > 0; i -= i & (-i))
        s += bit[i];
    return s;
}

int main() {
    scanf("%d", &n);
    memset(bit, 0, sizeof(bit));

    for (int i = 1; i <= n; i++) {
        scanf("%d", &p[i]);
        update(i, p[i]);
    }

    char op;
    int idx;
    while (scanf(" %c %d", &op, &idx) == 2) {
        if (op == 'a') {
            update(idx, -p[idx]);
            p[idx] = 0;
        } else {
            printf("%d\n", query(idx - 1));
        }
    }

    return 0;
}
