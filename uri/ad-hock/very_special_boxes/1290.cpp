#include <iostream>
#include <map>
#include <algorithm>
using namespace std;

int main() {
    int n, m;
    while (cin >> n >> m && n) {
        int x, y, z;
        cin >> x >> y >> z;
        int item[3] = {x, y, z};
        sort(item, item + 3);
        int vol_item = item[0] * item[1] * item[2];
        
        map<tuple<int,int,int>, int> conteo;
        
        for (int i = 0; i < m; i++) {
            int a, b, c;
            cin >> a >> b >> c;
            int box[3] = {a, b, c};
            sort(box, box + 3);
            
            if (box[0] >= item[0] && box[1] >= item[1] && box[2] >= item[2]) {
                conteo[{box[0], box[1], box[2]}]++;
            }
        }
        
        int mejor = -1;
        for (auto& p : conteo) {
            if (p.second >= n) {
                int vol_caja = get<0>(p.first) * get<1>(p.first) * get<2>(p.first);
                int espacio = vol_caja - vol_item;
                if (mejor == -1 || espacio < mejor) {
                    mejor = espacio;
                }
            }
        }
        
        if (mejor == -1) {
            cout << "impossible" << endl;
        } else {
            cout << mejor << endl;
        }
    }
    return 0;
}
