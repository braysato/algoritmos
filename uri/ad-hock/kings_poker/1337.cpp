#include <iostream>
#include <algorithm>

using namespace std;

int main() {
    int a, b, c;
    
    while (cin >> a >> b >> c && (a || b || c)) {
        int cards[3] = {a, b, c};
        sort(cards, cards + 3);
        
        if (cards[0] == cards[1] && cards[1] == cards[2]) {
            int rank = cards[0];
            
            if (rank == 13) {
                cout << "*" << endl;
            } else {
                cout << (rank + 1) << " " << (rank + 1) << " " << (rank + 1) << endl;
            }
        } else if (cards[0] == cards[1] || cards[1] == cards[2]) {
            int pairRank, single;
            if (cards[0] == cards[1]) {
                pairRank = cards[0];
                single = cards[2];
            } else {
                pairRank = cards[1];
                single = cards[0];
            }
            
            int nextCard = single + 1;
            if (nextCard == pairRank) {
                nextCard++;
            }
            
            if (nextCard <= 13) {
                cout << min(nextCard, pairRank) << " " << pairRank << " " << max(nextCard, pairRank) << endl;
            } else if (pairRank < 13) {
                cout << "1 " << (pairRank + 1) << " " << (pairRank + 1) << endl;
            } else {
                cout << "1 1 1" << endl;
            }
        } else {
            cout << "1 1 2" << endl;
        }
    }
    
    return 0;
}
