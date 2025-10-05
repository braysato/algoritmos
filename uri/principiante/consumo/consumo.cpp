#include <iostream>
#include <iomanip>
using namespace std;

int main() {
    int X;
    float Y;
    
    cin >> X >> Y;
    
    float consumo = (float)X / Y;
    
    cout << fixed << setprecision(3) << consumo << " km/l" << endl;
    
    return 0;
}