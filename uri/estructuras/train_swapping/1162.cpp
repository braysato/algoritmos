#include <iostream>
#include <vector>
using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    
    int n;
    cin >> n;
    
    while (n--) {
        int l;
        cin >> l;
        
        vector<int> vagones(l);
        for (int i = 0; i < l; i++) {
            cin >> vagones[i];
        }
        
        int swaps = 0;
        for (int i = 0; i < l; i++) {
            for (int j = i + 1; j < l; j++) {
                if (vagones[i] > vagones[j]) {
                    swaps++;
                }
            }
        }
        
        cout << "Optimal train swapping takes " << swaps << " swaps.\n";
    }
    
    return 0;
}
