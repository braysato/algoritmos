#include <cstdio>
#include <cstring>

int cnt[231];

int main() {
    int nc;
    scanf("%d", &nc);
    
    while (nc--) {
        int n;
        scanf("%d", &n);
        
        memset(cnt, 0, sizeof(cnt));
        
        for (int i = 0; i < n; i++) {
            int h;
            scanf("%d", &h);
            cnt[h]++;
        }
        
        int first = 1;
        for (int h = 20; h <= 230; h++) {
            while (cnt[h]--) {
                if (!first) putchar(' ');
                printf("%d", h);
                first = 0;
            }
        }
        putchar('\n');
    }
    
    return 0;
}
