#include <iostream>
#include <vector>
#include <sstream>
using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    
    int n;
    cin >> n;
    
    bool primero = true;
    
    while (n--) {
        int m, c;
        cin >> m >> c;
        
        vector<vector<int>> tabla(m);
        
        for (int i = 0; i < c; i++) {
            int clave;
            cin >> clave;
            int indice = clave % m;
            tabla[indice].push_back(clave);
        }
        
        if (!primero) {
            cout << '\n';
        }
        primero = false;
        
        for (int i = 0; i < m; i++) {
            cout << i << " -> ";
            for (int j = 0; j < tabla[i].size(); j++) {
                cout << tabla[i][j] << " -> ";
            }
            cout << "\\\n";
        }
    }
    
    return 0;
}
