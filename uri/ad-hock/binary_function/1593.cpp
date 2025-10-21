#include <iostream>
#include <string>

using namespace std;

string divideByTwo(string num, int &remainder) {
    string result = "";
    int carry = 0;
    
    for (int i = 0; i < num.length(); i++) {
        int current = carry * 10 + (num[i] - '0');
        result += (current / 2) + '0';
        carry = current % 2;
    }
    
    remainder = carry;
    
    int start = 0;
    while (start < result.length() && result[start] == '0') {
        start++;
    }
    
    if (start == result.length()) {
        return "0";
    }
    
    return result.substr(start);
}

int countOnes(string num) {
    int count = 0;
    
    while (num != "0") {
        int remainder;
        num = divideByTwo(num, remainder);
        if (remainder == 1) {
            count++;
        }
    }
    
    return count;
}

int main() {
    int t;
    cin >> t;
    
    while (t--) {
        string num;
        cin >> num;
        cout << countOnes(num) << endl;
    }
    
    return 0;
}
