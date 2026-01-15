#include <iostream>
#include <string>

using namespace std;

int main() {
    int n;
    cin >> n;
    cin.ignore();
    
    string message;
    int total_lines = 0;
    
    while(getline(cin, message)) {
        while(!message.empty() && message.back() == ' ') {
            message.pop_back();
        }
        
        if(message.empty()) {
            continue;
        }
        
        int lines = 1;
        int char_count = 0;
        int i = 0;
        int len = (int)message.size();
        
        while(i < len) {
            char_count++;
            i++;
            
            if(char_count == n && i < len) {
                while(i < len && message[i] == ' ') {
                    i++;
                }
                if(i < len) {
                    lines++;
                    char_count = 0;
                }
            }
        }
        
        total_lines += lines;
    }
    
    cout << total_lines << endl;
    
    return 0;
}
