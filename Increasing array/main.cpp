#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
int main()
{
    //freopen("in", "r", stdin);
    int n, k, l;
    ll ans = 0;
    cin >> n;
    cin >> l;
    for (int i = 1; i < n; i++)
    {
        cin >> k;
        if (k < l)
            ans += (ll)(l - k);
        l = max(l, k);
    }
    printf("%lld", ans);
    return 0;
}