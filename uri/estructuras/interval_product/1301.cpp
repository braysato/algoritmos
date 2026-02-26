#include <iostream>
#include <vector>
#include <string>
using namespace std;

vector<int> tree;
vector<int> arr;

int signo(int x) {
    if (x > 0) return 1;
    if (x < 0) return -1;
    return 0;
}

void build(int node, int start, int end) {
    if (start == end) {
        tree[node] = arr[start];
    } else {
        int mid = (start + end) / 2;
        build(2 * node, start, mid);
        build(2 * node + 1, mid + 1, end);
        tree[node] = tree[2 * node] * tree[2 * node + 1];
    }
}

void update(int node, int start, int end, int idx, int val) {
    if (start == end) {
        arr[idx] = val;
        tree[node] = val;
    } else {
        int mid = (start + end) / 2;
        if (idx <= mid) {
            update(2 * node, start, mid, idx, val);
        } else {
            update(2 * node + 1, mid + 1, end, idx, val);
        }
        tree[node] = tree[2 * node] * tree[2 * node + 1];
    }
}

int query(int node, int start, int end, int l, int r) {
    if (r < start || end < l) {
        return 1;
    }
    if (l <= start && end <= r) {
        return tree[node];
    }
    int mid = (start + end) / 2;
    int p1 = query(2 * node, start, mid, l, r);
    int p2 = query(2 * node + 1, mid + 1, end, l, r);
    return p1 * p2;
}

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    
    int n, k;
    while (cin >> n >> k) {
        arr.assign(n + 1, 0);
        tree.assign(4 * n, 0);
        
        for (int i = 1; i <= n; i++) {
            int x;
            cin >> x;
            arr[i] = signo(x);
        }
        
        build(1, 1, n);
        
        string resultado;
        
        for (int i = 0; i < k; i++) {
            char cmd;
            cin >> cmd;
            
            if (cmd == 'C') {
                int idx, val;
                cin >> idx >> val;
                update(1, 1, n, idx, signo(val));
            } else {
                int l, r;
                cin >> l >> r;
                int prod = query(1, 1, n, l, r);
                if (prod > 0) resultado += '+';
                else if (prod < 0) resultado += '-';
                else resultado += '0';
            }
        }
        
        cout << resultado << '\n';
    }
    
    return 0;
}
