#include <iostream>
#include <string>
using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    
    int n;
    cin >> n;
    cin.ignore();
    
    while (n--) {
        string linea;
        getline(cin, linea);
        
        int pila = 0;
        int diamantes = 0;
        
        for (char c : linea) {
            if (c == '<') {
                pila++;
            } else if (c == '>') {
                if (pila > 0) {
                    pila--;
                    diamantes++;
                }
            }
        }
        
        cout << diamantes << '\n';
    }
    
    return 0;
}
