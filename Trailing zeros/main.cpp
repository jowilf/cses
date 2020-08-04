#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
int main()
{
    //freopen("in", "r", stdin);
    int n, k = 1, ans = 0;
    cin >> n;
    while (pow(5, k) <= n)
    {
        ans += n / (pow(5, k));
        k += 1;
    }
    printf("%d", ans);
    return 0;
}