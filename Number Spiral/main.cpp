#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
int main()
{
    //freopen("in", "r", stdin);
    ll t, y, x, s, ans, ss;
    cin >> t;
    while (t--)
    {
        cin >> y >> x;
        ans = 0;
        if (x == y)
            ans = x * y - (y - 1);
        else
        {
            s = max(y, x);
            ss = s * s - (s - 1);
            if (s % 2 == 0)
            {
                if (s == x)
                    ans = ss - (s - y);
                else
                    ans = ss + (s - x);
            }
            else
            {
                if (s == x)
                    ans = ss + (s - y);
                else
                    ans = ss - (s - x);
            }
        }
        printf("%lld\n", ans);
    }
    return 0;
}