#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

struct Carta {
    int rango;
    char palo;
    
    int valorPalo() const {
        if (palo == 'C') return 0;
        if (palo == 'D') return 1;
        if (palo == 'H') return 2;
        return 3;
    }
    
    bool operator<(const Carta& o) const {
        if (rango != o.rango) return rango < o.rango;
        return valorPalo() < o.valorPalo();
    }
    
    bool coincide(const Carta& o) const {
        return rango == o.rango || palo == o.palo;
    }
};

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);
    
    int p, m, n;
    while (cin >> p >> m >> n && (p || m || n)) {
        vector<Carta> deck(n);
        for (int i = 0; i < n; i++) {
            cin >> deck[i].rango >> deck[i].palo;
        }
        
        vector<vector<Carta>> manos(p);
        int idx = 0;
        for (int i = 0; i < p; i++) {
            for (int j = 0; j < m; j++) {
                manos[i].push_back(deck[idx++]);
            }
        }
        
        Carta top = deck[idx++];
        vector<Carta> stock;
        for (int i = idx; i < n; i++) {
            stock.push_back(deck[i]);
        }
        reverse(stock.begin(), stock.end());
        
        int dir = 1;
        int jugador = 0;
        
        if (top.rango == 12) {
            dir = -dir;
        }
        
        int penalidad = 0;
        bool skipTurno = false;
        if (top.rango == 7) {
            penalidad = 2;
            skipTurno = true;
        } else if (top.rango == 1) {
            penalidad = 1;
            skipTurno = true;
        } else if (top.rango == 11) {
            skipTurno = true;
        }
        
        while (true) {
            for (int i = 0; i < penalidad && !stock.empty(); i++) {
                manos[jugador].push_back(stock.back());
                stock.pop_back();
            }
            penalidad = 0;
            
            if (skipTurno) {
                skipTurno = false;
                jugador = (jugador + dir + p) % p;
                continue;
            }
            
            int mejor = -1;
            for (int i = 0; i < manos[jugador].size(); i++) {
                if (manos[jugador][i].coincide(top)) {
                    if (mejor == -1 || manos[jugador][mejor] < manos[jugador][i]) {
                        mejor = i;
                    }
                }
            }
            
            if (mejor == -1 && !stock.empty()) {
                Carta robada = stock.back();
                stock.pop_back();
                manos[jugador].push_back(robada);
                if (robada.coincide(top)) {
                    mejor = manos[jugador].size() - 1;
                }
            }
            
            if (mejor != -1) {
                top = manos[jugador][mejor];
                manos[jugador].erase(manos[jugador].begin() + mejor);
                
                if (manos[jugador].empty()) {
                    cout << jugador + 1 << '\n';
                    break;
                }
            
                if (top.rango == 12) {
                    dir = -dir;
                } else if (top.rango == 7) {
                    penalidad = 2;
                    skipTurno = true;
                } else if (top.rango == 1) {
                    penalidad = 1;
                    skipTurno = true;
                } else if (top.rango == 11) {
                    skipTurno = true;
                }
            }
            
            jugador = (jugador + dir + p) % p;
        }
    }
    
    return 0;
}
