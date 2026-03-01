#include <iostream>
#include <vector>
using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    
    int t;
    cin >> t;
    
    while (t--) {
        int n;
        cin >> n;
        
        vector<int> arr(n + 1);
        vector<bool> visitado(n + 1, false);
        
        for (int i = 1; i <= n; i++) {
            cin >> arr[i];
        }
        
        int ciclos = 0;
        
        for (int i = 1; i <= n; i++) {
            if (!visitado[i]) {
                ciclos++;
                int j = i;
                while (!visitado[j]) {
                    visitado[j] = true;
                    j = arr[j];
                }
            }
        }
        
        cout << n - ciclos << '\n';
    }
    
    return 0;
}
