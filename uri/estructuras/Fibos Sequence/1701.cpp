#include <cstdio>
#include <cstring>

typedef long long ll;
const ll MOD = 1000000007;

struct Mat2 {
    ll a[2][2];
    Mat2() { memset(a, 0, sizeof(a)); }
};

Mat2 mul2(const Mat2& A, const Mat2& B) {
    Mat2 C;
    for (int i = 0; i < 2; i++)
        for (int j = 0; j < 2; j++)
            for (int k = 0; k < 2; k++)
                C.a[i][j] = (C.a[i][j] + A.a[i][k] * B.a[k][j]) % MOD;
    return C;
}

Mat2 pow2(Mat2 M, ll n) {
    Mat2 R;
    R.a[0][0] = R.a[1][1] = 1;
    while (n > 0) {
        if (n & 1) R = mul2(R, M);
        M = mul2(M, M);
        n >>= 1;
    }
    return R;
}

void fib(ll n, ll &fn, ll &fn1) {
    if (n == 0) { fn = 0; fn1 = 1; return; }
    Mat2 base;
    base.a[0][0] = base.a[0][1] = base.a[1][0] = 1;
    Mat2 R = pow2(base, n);
    fn = R.a[1][0];
    fn1 = R.a[0][0];
}

struct Mat5 {
    ll a[5][5];
    Mat5() { memset(a, 0, sizeof(a)); }
};

Mat5 mul5(const Mat5& A, const Mat5& B) {
    Mat5 C;
    for (int i = 0; i < 5; i++)
        for (int j = 0; j < 5; j++)
            for (int k = 0; k < 5; k++)
                C.a[i][j] = (C.a[i][j] + A.a[i][k] * B.a[k][j]) % MOD;
    return C;
}

Mat5 pow5(Mat5 M, ll n) {
    Mat5 R;
    for (int i = 0; i < 5; i++) R.a[i][i] = 1;
    while (n > 0) {
        if (n & 1) R = mul5(R, M);
        M = mul5(M, M);
        n >>= 1;
    }
    return R;
}

int main() {
    ll A, B, N;
    while (scanf("%lld %lld %lld", &A, &B, &N) == 3) {
        if (!A && !B && !N) break;

        ll fa, fa1, fb, fb1;
        fib(A, fa, fa1);
        fib(B, fb, fb1);

        ll p = fa * fb % MOD;
        ll q = fa1 * fb % MOD;
        ll r = fa * fb1 % MOD;
        ll s = fa1 * fb1 % MOD;
        ll S = p;

        if (N == 1) {
            printf("%lld\n", S);
            continue;
        }

        // Transition: p'=s, q'=r+s, r'=q+s, s'=p+q+r+s, S'=S+s
        Mat5 T;
        T.a[0][3] = 1;
        T.a[1][2] = 1; T.a[1][3] = 1;
        T.a[2][1] = 1; T.a[2][3] = 1;
        T.a[3][0] = 1; T.a[3][1] = 1; T.a[3][2] = 1; T.a[3][3] = 1;
        T.a[4][3] = 1; T.a[4][4] = 1;

        Mat5 TN = pow5(T, N - 1);

        ll vec[5] = {p, q, r, s, S};
        ll ans = 0;
        for (int j = 0; j < 5; j++)
            ans = (ans + TN.a[4][j] * vec[j]) % MOD;

        printf("%lld\n", ans);
    }

    return 0;
}
