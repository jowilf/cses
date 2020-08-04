#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define N 1000005
int main()
{
    //freopen("in", "r", stdin);
    int n;
    cin >> n;
    int choosen[n + 3];
    ll _sum = ((ll)(n + 1) * n) / 2;
    if (_sum % 2 != 0)
        printf("NO\n");
    else
    {
        printf("YES\n");
        memset(choosen, 0, sizeof choosen);
        ll sum2 = _sum / 2,
           isum = 0;
        vector<int> setI;
        for (int i = n; i >= 1; i--)
        {
            if (isum + i < sum2)
            {
                setI.push_back(i);
                isum += i;
                choosen[i] = 1;
            }
            else
            {
                setI.push_back((int)(sum2 - isum));
                choosen[(int)(sum2 - isum)] = 1;
                break;
            }
        }
        printf("%d\n", setI.size());
        for (auto v : setI)
            printf("%d ", v);
        printf("\n%d\n", n - setI.size());
        ll ss = 0;
        for (int i = 1; i <= n; i++)
        {
            if (!choosen[i])
            {
                printf("%d ", i);
                ss += i;
            }
        }
    }
    return 0;
}