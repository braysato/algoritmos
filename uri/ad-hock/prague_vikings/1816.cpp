#include <iostream>
#include <vector>

using namespace std;

int main() {
    int m;
    int instance = 1;
    
    while (cin >> m && m != 0) {
        vector<char> alphabet;
        for (char c = 'A'; c <= 'Z'; c++) {
            alphabet.push_back(c);
        }
        
        string result = "";
        
        for (int i = 0; i < m; i++) {
            int pos;
            cin >> pos;
            pos--;
            
            char letter = alphabet[pos];
            result += letter;
            
            alphabet.erase(alphabet.begin() + pos);
            alphabet.insert(alphabet.begin(), letter);
        }
        
        cout << "Instancia " << instance << endl;
        cout << result << endl;
        cout << endl;
        
        instance++;
    }
    
    return 0;
}

