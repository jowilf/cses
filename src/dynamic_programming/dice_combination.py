n = int(input())
cnt = [(1 if (i in range(1, 7)) else 0) for i in range(n + 1)]

for i in range(n + 1):
    for j in range(1, 7):
        if (i - j) > 0:
            cnt[i] += cnt[i - j]
            cnt[i] %= 10**9 + 7
print(cnt[n])
# TLE
