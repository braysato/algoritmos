#include <iostream>
#include <string>
#include <map>
#include <iomanip>
using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    
    int n;
    cin >> n;
    cin.ignore();
    cin.ignore();
    
    bool primero = true;
    
    while (n--) {
        map<string, int> especies;
        int total = 0;
        string linea;
        
        while (getline(cin, linea) && !linea.empty()) {
            especies[linea]++;
            total++;
        }
        
        if (!primero) {
            cout << '\n';
        }
        primero = false;
        
        cout << fixed << setprecision(4);
        for (auto& p : especies) {
            double porcentaje = (p.second * 100.0) / total;
            cout << p.first << ' ' << porcentaje << '\n';
        }
    }
    
    return 0;
}
