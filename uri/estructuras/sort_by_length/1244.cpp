#include <iostream>
#include <string>
#include <vector>
#include <sstream>
#include <algorithm>
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
        
        vector<string> palabras;
        stringstream ss(linea);
        string palabra;
        
        while (ss >> palabra) {
            palabras.push_back(palabra);
        }
        
        stable_sort(palabras.begin(), palabras.end(), [](const string& a, const string& b) {
            return a.length() > b.length();
        });
        
        for (int i = 0; i < palabras.size(); i++) {
            if (i > 0) cout << " ";
            cout << palabras[i];
        }
        cout << '\n';
    }
    
    return 0;
}
