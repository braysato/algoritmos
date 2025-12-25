#include <iostream>
#include <vector>

using namespace std;

struct Direction {
    int dx, dy, dz;
};

bool check_win(const vector<vector<vector<char>>> &board, int n, int x, int y, int z, char player) {
    static const Direction dirs[] = {
        {1, 0, 0}, {0, 1, 0}, {0, 0, 1},
        {1, 1, 0}, {1, -1, 0}, {1, 0, 1}, {1, 0, -1},
        {0, 1, 1}, {0, 1, -1},
        {1, 1, 1}, {1, 1, -1}, {1, -1, 1}, {1, -1, -1}
    };
    for (const auto &d : dirs) {
        int count = 1;
        int nx = x + d.dx;
        int ny = y + d.dy;
        int nz = z + d.dz;
        while (nx >= 0 && nx < n && ny >= 0 && ny < n && nz >= 0 && nz < n && board[nx][ny][nz] == player) {
            ++count;
            nx += d.dx;
            ny += d.dy;
            nz += d.dz;
        }
        nx = x - d.dx;
        ny = y - d.dy;
        nz = z - d.dz;
        while (nx >= 0 && nx < n && ny >= 0 && ny < n && nz >= 0 && nz < n && board[nx][ny][nz] == player) {
            ++count;
            nx -= d.dx;
            ny -= d.dy;
            nz -= d.dz;
        }
        if (count >= n) {
            return true;
        }
    }
    return false;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    int instance = 1;
    while (cin >> n) {
        if (n == 0) {
            break;
        }
        vector<vector<vector<char>>> board(n, vector<vector<char>>(n, vector<char>(n, '.')));
        vector<vector<int>> height(n, vector<int>(n, 0));
        char winner = '.';
        int total_moves = n * n * n;
        for (int move = 0; move < total_moves; ++move) {
            int i, j;
            cin >> i >> j;
            int x = i - 1;
            int y = j - 1;
            int z = height[x][y];
            height[x][y]++;
            char player = (move % 2 == 0) ? 'B' : 'A';
            board[x][y][z] = player;
            if (winner == '.' && check_win(board, n, x, y, z, player)) {
                winner = player;
            }
        }
        cout << "Instancia " << instance << '\n';
        if (winner == 'B') {
            cout << "Branco ganhou" << '\n';
        } else if (winner == 'A') {
            cout << "Azul ganhou" << '\n';
        } else {
            cout << "Empate" << '\n';
        }
        cout << '\n';
        ++instance;
    }
    return 0;
}
