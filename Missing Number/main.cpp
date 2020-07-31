#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
int main()
{
    //freopen("in", "r", stdin);
    int n, k;
    cin >> n;
    ll s = (n * (n + 1)) / 2;
    for (int i = 0; i < n-1; i++)
    {
        cin >> k;
        s -= k;
    }
    printf("%lld", s);
    return 0;
}