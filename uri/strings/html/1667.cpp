#include <iostream>
#include <string>
using namespace std;

int main() {
    string token;
    int pos = 0;
    
    while (cin >> token) {
        if (token == "<br>") {
            cout << '\n';
            pos = 0;
        } else if (token == "<hr>") {
            if (pos > 0) {
                cout << '\n';
                pos = 0;
            }
            cout << string(80, '-') << '\n';
        } else {
            int len = token.length();
            if (pos == 0) {
                cout << token;
                pos = len;
            } else if (pos + 1 + len <= 80) {
                cout << " " << token;
                pos += 1 + len;
            } else {
                cout << '\n' << token;
                pos = len;
            }
        }
    }
    
    if (pos > 0) {
        cout << '\n';
    }
    
    return 0;
}
