#include <iostream>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int startHour, startMinute, endHour, endMinute;
    while (cin >> startHour >> startMinute >> endHour >> endMinute) {
        int startTotal = startHour * 60 + startMinute;
        int endTotal = endHour * 60 + endMinute;
        if (endTotal <= startTotal) {
            endTotal += 24 * 60;
        }
        int duration = endTotal - startTotal;
        int hours = duration / 60;
        int minutes = duration % 60;
        cout << "O JOGO DUROU " << hours << " HORA(S) E " << minutes << " MINUTO(S)" << '\n';
    }

    return 0;
}
