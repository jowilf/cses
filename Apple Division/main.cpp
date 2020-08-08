#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define N 1000005
int n, t[21];
ll ans = 20e9 + 1, s1 = 0, s2 = 0;
void solve(int i, int grp)
{
    if (i == n)
    {
        //printf("%lld - %lld\n", s1, s2);
        ans = min(ans, abs(s1 - s2));
        return;
    }
    if (grp == 1)
        s1 += t[i];
    else
        s2 += t[i];
    solve(i + 1, 1);
    s1 -= t[i + 1];
    solve(i + 1, 2);
    s2 -= t[i + 1];
}
int main()
{
    //freopen("in", "r", stdin);
    memset(t, 0, sizeof t);
    cin >> n;
    for (int i = 0; i < n; i++)
        cin >> t[i];
    solve(0, 1);
    s1 = s2 = 0;
    solve(0, 2);
    printf("%lld", ans);
    return 0;
}