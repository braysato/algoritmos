#include <iostream>
#include <vector>
#include <string>

using namespace std;

int main() {
    int n, m;
    
    while (cin >> n >> m && (n != 0 || m != 0)) {
        vector<string> imagen(n);
        
        for (int i = 0; i < n; i++) {
            cin >> imagen[i];
        }
        
        int a, b;
        cin >> a >> b;
        
        int factorV = a / n;
        int factorH = b / m;
        
        for (int i = 0; i < n; i++) {
            string lineaExpandida = "";
            for (int j = 0; j < m; j++) {
                for (int k = 0; k < factorH; k++) {
                    lineaExpandida += imagen[i][j];
                }
            }
            
            for (int k = 0; k < factorV; k++) {
                cout << lineaExpandida << endl;
            }
        }
        
        cout << endl;
    }
    
    return 0;
}
