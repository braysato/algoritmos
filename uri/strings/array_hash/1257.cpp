#include <iostream>
#include <string>

using namespace std;

int main() {
    int n;
    cin >> n;
    
    for (int t = 0; t < n; t++) {
        int l;
        cin >> l;
        cin.ignore();
        
        int hash = 0;
        
        for (int i = 0; i < l; i++) {
            string line;
            getline(cin, line);
            
            for (int j = 0; j < line.length(); j++) {
                int value = (line[j] - 'A') + i + j;
                hash += value;
            }
        }
        
        cout << hash << endl;
    }
    
    return 0;
}
