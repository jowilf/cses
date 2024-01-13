print(10**9)
n, m = map(int, input().split())
arr = list(map(int, input().split()))
dp = [[0 for _ in range(m + 1)] for __ in range(n)]
if arr[0] == 0:
    for k in range(1, m + 1):
        dp[0][k] = 1
else:
    dp[0][arr[0]] = 1

for i in range(1, n):
    for k in range(1, m + 1) if arr[i] == 0 else [arr[i]]:
        dp[i][k] += dp[i - 1][k]
        if k > 1:
            dp[i][k] += dp[i - 1][k - 1]
        if k < m:
            dp[i][k] += dp[i - 1][k + 1]
        dp[i][k] %= 10**9 + 7

# for i in range(1, m + 1):
#     print([dp[j][i] for j in range(n)])
# print([dp[n - 1][k] for k in range(1, m + 1)])
print(sum([(dp[n - 1][k] % (10**9 + 7)) for k in range(1, m + 1)]) % (10**9 + 7))
