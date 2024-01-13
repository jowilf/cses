from math import log2

n, q = map(int, input().split())

MAX_K_POWER = 30
succ = [-1] + list(map(int, input().split()))

dp = [[None for _ in range(MAX_K_POWER)] for __ in range(n + 1)]

for k in range(MAX_K_POWER):
    for i in range(1, n + 1):
        if k == 0:
            dp[i][k] = succ[i]
        else:
            dp[i][k] = dp[dp[i][k - 1]][k - 1]
    # print([dp[v][k] for v in range(1, n + 1)])

for i in range(q):
    x, k = map(int, input().split())
    while k > 0:
        # print(x, k, dp[x])
        p2 = int(log2(k))
        x = dp[x][p2]
        k -= 2**p2
    print(x)

"""
10 1
2 3 4 5 6 7 8 9 10 1
2 10
6 5
6 6
3 7
3 1
9 6
9 8
7 9
1 10
6 6
"""
