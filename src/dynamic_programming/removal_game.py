def get_score(arr, l, r, dp):
    if l == r:
        return arr[l], 0
    if dp[l][r] is not None:
        return dp[l][r]
    s1 = get_score(arr, l + 1, r, dp)
    s2 = get_score(arr, l, r - 1, dp)
    if (arr[l] + s1[1]) > arr[r] + s2[1]:
        dp[l][r] = arr[l] + s1[1], s1[0]
    else:
        dp[l][r] = arr[r] + s2[1], s2[0]
    return dp[l][r]


n = int(input())
dp = [[None for _ in range(n)] for __ in range(n + 1)]
arr = list(map(int, input().split()))
for i in range(n):
    dp[i][i] = arr[i], 0
for l in range(n - 1, -1, -1):
    for r in range(l + 1, n):
        s1 = dp[l + 1][r]
        s2 = dp[l][r - 1]
        if (arr[l] + s1[1]) > arr[r] + s2[1]:
            dp[l][r] = arr[l] + s1[1], s1[0]
        else:
            dp[l][r] = arr[r] + s2[1], s2[0]

print(dp[0][n - 1][0])
# print(dp)
