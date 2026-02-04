#include <iostream>
using namespace std;

long long mcd(long long a, long long b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) {
        long long t = b;
        b = a % b;
        a = t;
    }
    return a;
}

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    
    int n;
    cin >> n;
    
    while (n--) {
        long long n1, d1, n2, d2;
        char barra, op;
        
        cin >> n1 >> barra >> d1 >> op >> n2 >> barra >> d2;
        
        long long num, den;
        
        if (op == '+') {
            num = n1 * d2 + n2 * d1;
            den = d1 * d2;
        } else if (op == '-') {
            num = n1 * d2 - n2 * d1;
            den = d1 * d2;
        } else if (op == '*') {
            num = n1 * n2;
            den = d1 * d2;
        } else {
            num = n1 * d2;
            den = n2 * d1;
        }
        
        long long g = mcd(num, den);
        long long numSimp = num / g;
        long long denSimp = den / g;
        
        if (denSimp < 0) {
            numSimp = -numSimp;
            denSimp = -denSimp;
        }
        
        cout << num << "/" << den << " = " << numSimp << "/" << denSimp << '\n';
    }
    
    return 0;
}
