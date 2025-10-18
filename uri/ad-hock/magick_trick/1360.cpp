#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

int getValueIndex(char value) {
    if (value >= '1' && value <= '9') return value - '1';
    if (value == 'T') return 9;
    if (value == 'J') return 10;
    if (value == 'Q') return 11;
    if (value == 'K') return 12;
    return 0;
}

char getValueChar(int index) {
    if (index >= 0 && index <= 8) return '1' + index;
    if (index == 9) return 'T';
    if (index == 10) return 'J';
    if (index == 11) return 'Q';
    if (index == 12) return 'K';
    return '1';
}

int getSuitIndex(char suit) {
    if (suit == 'H') return 0;
    if (suit == 'C') return 1;
    if (suit == 'D') return 2;
    if (suit == 'S') return 3;
    return 0;
}

bool compareCards(string a, string b) {
    char valueA = a[0], suitA = a[1];
    char valueB = b[0], suitB = b[1];
    
    if (getSuitIndex(suitA) != getSuitIndex(suitB)) {
        return getSuitIndex(suitA) < getSuitIndex(suitB);
    }
    return getValueIndex(valueA) < getValueIndex(valueB);
}

int getOffset(vector<string> ordered, vector<string> original) {
    if (original[0] == ordered[0] && original[1] == ordered[1] && original[2] == ordered[2]) return 1;
    if (original[0] == ordered[0] && original[1] == ordered[2] && original[2] == ordered[1]) return 2;
    if (original[0] == ordered[1] && original[1] == ordered[0] && original[2] == ordered[2]) return 3;
    if (original[0] == ordered[1] && original[1] == ordered[2] && original[2] == ordered[0]) return 4;
    if (original[0] == ordered[2] && original[1] == ordered[0] && original[2] == ordered[1]) return 5;
    if (original[0] == ordered[2] && original[1] == ordered[1] && original[2] == ordered[0]) return 6;
    return 1;
}

int main() {
    int n;
    cin >> n;
    
    for (int i = 0; i < n; i++) {
        string card1, card2, card3, card4;
        cin >> card1 >> card2 >> card3 >> card4;
        
        char suit = card1[1];
        int valueIndex = getValueIndex(card1[0]);
        
        vector<string> lastThree = {card2, card3, card4};
        vector<string> ordered = lastThree;
        sort(ordered.begin(), ordered.end(), compareCards);
        
        int offset = getOffset(ordered, lastThree);
        
        int hiddenValueIndex = (valueIndex + offset) % 13;
        char hiddenValue = getValueChar(hiddenValueIndex);
        
        cout << hiddenValue << suit << endl;
    }
    
    return 0;
}
