#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
int main()
{
    //freopen("in", "r", stdin);
    ll n;
    cin >> n;
    printf("%lld ", n);
    while (n > 1)
    {
        if (n % 2 == 0)
            n /= 2;
        else
            n = n * 3 + 1;
        printf("%lld ", n);
    }
    return 0;
}