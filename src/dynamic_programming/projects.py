n = int(input())

days = dict()

inp = []
for i in range(n):
    a, b, s = map(int, input().split())
    days[a] = 0
    days[b] = 0
    inp.append((a, b, s))

for i, v in enumerate(sorted(days.keys())):
    days[v] = i
cnt = len(days)
print(days)
projs = [[] for _ in range(cnt)]
for p in inp:
    projs[days[p[1]]].append(p)

dp = [0 for _ in range(cnt + 1)]

for i in range(1, cnt):
    dp[i] = dp[i - 1]
    for p in projs[i]:
        dp[i] = max(dp[i], dp[days[p[0]] - 1] + p[2])
print(projs)
print(dp)
print(dp[cnt - 1])
