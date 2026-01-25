#include <iostream>
#include <string>
using namespace std;

int main() {
    int n;
    cin >> n;
    
    while (n--) {
        int k;
        cin >> k;
        
        string primera, actual;
        cin >> primera;
        
        bool misma_lengua = true;
        
        for (int i = 1; i < k; i++) {
            cin >> actual;
            if (actual != primera) {
                misma_lengua = false;
            }
        }
        
        if (misma_lengua) {
            cout << primera << endl;
        } else {
            cout << "ingles" << endl;
        }
    }
    
    return 0;
}
