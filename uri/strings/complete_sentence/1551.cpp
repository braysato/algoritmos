#include <iostream>
#include <string>
#include <set>

using namespace std;

int main() {
    int n;
    cin >> n;
    cin.ignore();
    
    for (int i = 0; i < n; i++) {
        string line;
        getline(cin, line);
        
        set<char> letters;
        
        for (char c : line) {
            if (c >= 'a' && c <= 'z') {
                letters.insert(c);
            }
        }
        
        int count = letters.size();
        
        if (count == 26) {
            cout << "frase completa" << endl;
        } else if (count >= 13) {
            cout << "frase quase completa" << endl;
        } else {
            cout << "frase mal elaborada" << endl;
        }
    }
    
    return 0;
}
