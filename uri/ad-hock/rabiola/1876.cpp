#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
    string vine;
    
    while (cin >> vine) {
        vector<int> segments;
        int count = 0;
        
        for (char c : vine) {
            if (c == 'o') {
                count++;
            } else {
                if (count > 0) {
                    segments.push_back(count);
                    count = 0;
                }
            }
        }
        
        if (count > 0) {
            segments.push_back(count);
        }
        
        int maxSize = 0;
        
        for (int i = 0; i < segments.size(); i++) {
            int size;
            if (i == 0 || i == segments.size() - 1) {
                size = segments[i];
            } else {
                size = segments[i] / 2;
            }
            maxSize = max(maxSize, size);
        }
        
        cout << maxSize << endl;
    }
    
    return 0;
}

