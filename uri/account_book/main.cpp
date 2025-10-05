#include <iostream>
#include <vector>
#include <string>
#include <bitset>

using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    
    int N, F;
    
    while (cin >> N >> F && (N != 0 || F != 0)) {
        vector<int> transactions(N);
        int total_sum = 0;
        
        for (int i = 0; i < N; ++i) {
            cin >> transactions[i];
            total_sum += transactions[i];
        }
        
        if (F > total_sum || F < -total_sum || (total_sum + F) % 2 != 0) {
            cout << "*\n";
            continue;
        }
        
        string result(N, '?');
        
        if (N <= 20) {
            bool can_achieve_target = false;
            vector<vector<bool>> valid_configs;
            
            for (int mask = 0; mask < (1 << N); ++mask) {
                int sum = 0;
                for (int i = 0; i < N; ++i) {
                    if (mask & (1 << i)) {
                        sum += transactions[i];
                    } else {
                        sum -= transactions[i];
                    }
                }
                
                if (sum == F) {
                    can_achieve_target = true;
                    vector<bool> config(N);
                    for (int i = 0; i < N; ++i) {
                        config[i] = (mask & (1 << i)) != 0;
                    }
                    valid_configs.push_back(config);
                }
            }
            
            if (!can_achieve_target) {
                cout << "*\n";
                continue;
            }
            
            for (int i = 0; i < N; ++i) {
                bool all_positive = true;
                bool all_negative = true;
                
                for (const auto& config : valid_configs) {
                    if (!config[i]) all_positive = false;
                    if (config[i]) all_negative = false;
                }
                
                if (all_positive) {
                    result[i] = '+';
                } else if (all_negative) {
                    result[i] = '-';
                } else {
                    result[i] = '?';
                }
            }
        } else {
            int half = N / 2;
            
            vector<pair<int, int>> first_half;
            for (int mask = 0; mask < (1 << half); ++mask) {
                int sum = 0;
                for (int i = 0; i < half; ++i) {
                    if (mask & (1 << i)) {
                        sum += transactions[i];
                    } else {
                        sum -= transactions[i];
                    }
                }
                first_half.push_back({sum, mask});
            }
            
            vector<pair<int, int>> second_half;
            for (int mask = 0; mask < (1 << (N - half)); ++mask) {
                int sum = 0;
                for (int i = 0; i < N - half; ++i) {
                    if (mask & (1 << i)) {
                        sum += transactions[half + i];
                    } else {
                        sum -= transactions[half + i];
                    }
                }
                second_half.push_back({sum, mask});
            }
            
            vector<pair<int, int>> valid_combinations;
            for (auto& p1 : first_half) {
                for (auto& p2 : second_half) {
                    if (p1.first + p2.first == F) {
                        valid_combinations.push_back({p1.second, p2.second});
                    }
                }
            }
            
            if (valid_combinations.empty()) {
                cout << "*\n";
                continue;
            }
            
            for (int i = 0; i < N; ++i) {
                bool all_positive = true;
                bool all_negative = true;
                
                for (auto& combo : valid_combinations) {
                    bool is_positive;
                    if (i < half) {
                        is_positive = (combo.first & (1 << i)) != 0;
                    } else {
                        is_positive = (combo.second & (1 << (i - half))) != 0;
                    }
                    
                    if (!is_positive) all_positive = false;
                    if (is_positive) all_negative = false;
                }
                
                if (all_positive) {
                    result[i] = '+';
                } else if (all_negative) {
                    result[i] = '-';
                } else {
                    result[i] = '?';
                }
            }
        }
        
        cout << result << "\n";
    }
    
    return 0;
}
