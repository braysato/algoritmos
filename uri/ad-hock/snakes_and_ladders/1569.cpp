#include <iostream>
#include <map>
#include <vector>

using namespace std;

int main() {
    int t;
    cin >> t;
    
    while (t--) {
        int a, b, c;
        cin >> a >> b >> c;
        
        map<int, int> board;
        
        for (int i = 0; i < b; i++) {
            int from, to;
            cin >> from >> to;
            board[from] = to;
        }
        
        vector<int> players(a + 1, 1);
        
        bool gameOver = false;
        int currentPlayer = 1;
        
        for (int i = 0; i < c; i++) {
            int roll;
            cin >> roll;
            
            if (!gameOver) {
                players[currentPlayer] += roll;
                
                if (players[currentPlayer] > 100) {
                    players[currentPlayer] = 100;
                }
                
                if (board.count(players[currentPlayer])) {
                    players[currentPlayer] = board[players[currentPlayer]];
                }
                
                if (players[currentPlayer] == 100) {
                    gameOver = true;
                }
                
                currentPlayer++;
                if (currentPlayer > a) {
                    currentPlayer = 1;
                }
            }
        }
        
        for (int i = 1; i <= a; i++) {
            cout << "Position of player " << i << " is " << players[i] << "." << endl;
        }
    }
    
    return 0;
}
