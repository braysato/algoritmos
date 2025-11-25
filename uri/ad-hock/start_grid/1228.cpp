#include <iostream>
#include <vector>
#include <map>

using namespace std;

int main() {
    int n;
    
    while (cin >> n) {
        vector<int> start(n);
        vector<int> finish(n);
        
        for (int i = 0; i < n; i++) {
            cin >> start[i];
        }
        
        for (int i = 0; i < n; i++) {
            cin >> finish[i];
        }
        
        map<int, int> startPosition;
        for (int i = 0; i < n; i++) {
            startPosition[start[i]] = i;
        }
        
        int overtakes = 0;
        for (int i = 0; i < n; i++) {
            for (int j = i + 1; j < n; j++) {
                if (startPosition[finish[i]] > startPosition[finish[j]]) {
                    overtakes++;
                }
            }
        }
        
        cout << overtakes << endl;
    }
    
    return 0;
}
