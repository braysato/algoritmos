#include <iostream>
#include <cstring>

using namespace std;

bool isValid(int grid[9][9]) {
    bool used[10];
    
    for (int i = 0; i < 9; i++) {
        memset(used, false, sizeof(used));
        for (int j = 0; j < 9; j++) {
            int num = grid[i][j];
            if (num < 1 || num > 9 || used[num]) {
                return false;
            }
            used[num] = true;
        }
    }
    
    for (int j = 0; j < 9; j++) {
        memset(used, false, sizeof(used));
        for (int i = 0; i < 9; i++) {
            int num = grid[i][j];
            if (num < 1 || num > 9 || used[num]) {
                return false;
            }
            used[num] = true;
        }
    }
    
    for (int block = 0; block < 9; block++) {
        memset(used, false, sizeof(used));
        int startRow = (block / 3) * 3;
        int startCol = (block % 3) * 3;
        
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                int num = grid[startRow + i][startCol + j];
                if (num < 1 || num > 9 || used[num]) {
                    return false;
                }
                used[num] = true;
            }
        }
    }
    
    return true;
}

int main() {
    int n;
    cin >> n;
    
    for (int instance = 1; instance <= n; instance++) {
        int grid[9][9];
        
        for (int i = 0; i < 9; i++) {
            for (int j = 0; j < 9; j++) {
                cin >> grid[i][j];
            }
        }
        
        cout << "Instancia " << instance << endl;
        if (isValid(grid)) {
            cout << "SIM" << endl;
        } else {
            cout << "NAO" << endl;
        }
        cout << endl;
    }
    
    return 0;
}
