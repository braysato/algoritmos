#include <iostream>
using namespace std;

int main() {
    int M;
    
    while(cin >> M) {
        if(M < 90 || M == 360) {
            cout << "Bom Dia!!" << endl;
        }
        else if(M >= 90 && M < 180) {
            cout << "Boa Tarde!!" << endl;
        }
        else if(M >= 180 && M < 270) {
            cout << "Boa Noite!!" << endl;
        }
        else if(M >= 270 && M < 360) {
            cout << "De Madrugada!!" << endl;
        }
    }
    
    return 0;
}