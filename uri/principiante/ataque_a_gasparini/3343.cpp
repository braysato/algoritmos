#include <iostream>
#include <vector>
using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    
    int n, x;
    cin >> n >> x;
    
    string titans;
    cin >> titans;
    
    int sizes[128];
    cin >> sizes['P'] >> sizes['M'] >> sizes['G'];
    sizes['p'] = sizes['P'];
    sizes['m'] = sizes['M'];
    sizes['g'] = sizes['G'];
    
    vector<int> walls;
    int last_check = 0;
    
    for (int i = 0; i < n; i++) {
        int titan_size = sizes[titans[i]];
        bool found = false;
        
        for (int j = last_check; j < walls.size(); j++) {
            if (walls[j] >= titan_size) {
                walls[j] -= titan_size;
                found = true;
                if (walls[j] == 0 && j == last_check) {
                    last_check++;
                }
                break;
            }
        }
        
        if (!found) {
            walls.push_back(x - titan_size);
        }
    }
    
    cout << walls.size() << endl;
    
    return 0;
}
