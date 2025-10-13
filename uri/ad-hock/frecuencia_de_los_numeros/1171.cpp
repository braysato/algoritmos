#include <iostream>

using namespace std;

int main() {
    int n;
    cin >> n;
    
    int frecuencia[2001] = {0};
    
    for (int i = 0; i < n; i++) {
        int x;
        cin >> x;
        frecuencia[x]++;
    }
    
    for (int i = 1; i <= 2000; i++) {
        if (frecuencia[i] > 0) {
            cout << i << " aparece " << frecuencia[i] << " vez(es)" << endl;
        }
    }
    
    return 0;
}
