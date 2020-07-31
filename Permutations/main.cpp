#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0);
    cout.tie(0);
    //freopen("in", "r", stdin);
    int n;
    cin >> n;
    if (n >= 2 && n <= 3)
        printf("NO SOLUTION");
    else
    {
        for (int i = 2; i <= n; i += 2)
            printf("%d ", i);
        for (int i = 1; i <= n; i += 2)
            printf("%d ", i);
    }
    return 0;
}