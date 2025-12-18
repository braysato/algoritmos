#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>
#include <array>
#include <limits>

using namespace std;

struct WordStats {
    int count = 0;
    int first_pos = 0;
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string line;
    while (getline(cin, line)) {
        if (line == ".") {
            break;
        }

        array<unordered_map<string, WordStats>, 26> stats;
        array<string, 26> chosen;
        array<long long, 26> best_gain{};
        array<int, 26> best_pos;
        best_pos.fill(numeric_limits<int>::max());

        vector<pair<bool, string>> tokens; 
        tokens.reserve(line.size());

        bool in_word = false;
        string buffer;
        string sep;
        int word_index = 0;

        auto flush_word = [&](void) {
            if (!buffer.empty()) {
                tokens.emplace_back(true, buffer);
                char initial = buffer[0];
                int idx = initial - 'a';
                auto &entry = stats[idx][buffer];
                if (entry.count == 0) {
                    entry.first_pos = word_index;
                }
                entry.count += 1;
                word_index += 1;
                buffer.clear();
            }
        };

        auto flush_sep = [&](void) {
            if (!sep.empty()) {
                tokens.emplace_back(false, sep);
                sep.clear();
            }
        };

        for (char ch : line) {
            if (ch >= 'a' && ch <= 'z') {
                if (!in_word) {
                    flush_sep();
                    in_word = true;
                }
                buffer.push_back(ch);
            } else {
                if (in_word) {
                    flush_word();
                    in_word = false;
                }
                sep.push_back(ch);
            }
        }
        if (in_word) {
            flush_word();
        }
        flush_sep();

        for (int idx = 0; idx < 26; ++idx) {
            for (const auto &kv : stats[idx]) {
                const string &word = kv.first;
                const WordStats &info = kv.second;
                int len = static_cast<int>(word.size());
                if (len <= 2) {
                    continue;
                }
                long long gain = static_cast<long long>(len - 2) * info.count;
                if (gain <= 0) {
                    continue;
                }
                if (gain > best_gain[idx] || (gain == best_gain[idx] && info.first_pos < best_pos[idx])) {
                    best_gain[idx] = gain;
                    best_pos[idx] = info.first_pos;
                    chosen[idx] = word;
                }
            }
        }

        string output;
        output.reserve(line.size());
        for (const auto &token : tokens) {
            if (token.first) {
                const string &word = token.second;
                int idx = word[0] - 'a';
                if (!chosen[idx].empty() && word == chosen[idx]) {
                    output.push_back(static_cast<char>('a' + idx));
                    output.push_back('.');
                } else {
                    output += word;
                }
            } else {
                output += token.second;
            }
        }

        cout << output << '\n';
        int count_choices = 0;
        for (int idx = 0; idx < 26; ++idx) {
            if (!chosen[idx].empty()) {
                count_choices += 1;
            }
        }
        cout << count_choices << '\n';
        for (int idx = 0; idx < 26; ++idx) {
            if (!chosen[idx].empty()) {
                char letter = static_cast<char>('a' + idx);
                cout << letter << ". = " << chosen[idx] << '\n';
            }
        }
    }

    return 0;
}
