#include <iostream>
#include <vector>
#include <set>

using namespace std;

set<int> possibleSums(const vector<int>& trans, int start, int end) {
    set<int> sums;
    sums.insert(0);
    
    for (int i = start; i <= end; i++) {
        set<int> newSums;
        for (int sum : sums) {
            newSums.insert(sum + trans[i]);
            newSums.insert(sum - trans[i]);
        }
        sums = newSums;
    }
    
    return sums;
}

int main() {
    int n, f;
    
    while (cin >> n >> f && (n != 0 || f != 0)) {
        vector<int> transactions(n);
        
        for (int i = 0; i < n; i++) {
            cin >> transactions[i];
        }
        
        set<int> allSums = possibleSums(transactions, 0, n - 1);
        
        if (allSums.find(f) == allSums.end()) {
            cout << "*" << endl;
            continue;
        }
        
        string result = "";
        
        for (int i = 0; i < n; i++) {
            set<int> before = (i > 0) ? possibleSums(transactions, 0, i - 1) : set<int>{0};
            set<int> after = (i < n - 1) ? possibleSums(transactions, i + 1, n - 1) : set<int>{0};
            
            bool canBePositive = false;
            bool canBeNegative = false;
            
            for (int b : before) {
                for (int a : after) {
                    if (b + transactions[i] + a == f) {
                        canBePositive = true;
                    }
                    if (b - transactions[i] + a == f) {
                        canBeNegative = true;
                    }
                }
            }
            
            if (canBePositive && !canBeNegative) {
                result += '+';
            } else if (!canBePositive && canBeNegative) {
                result += '-';
            } else {
                result += '?';
            }
        }
        
        cout << result << endl;
    }
    
    return 0;
}