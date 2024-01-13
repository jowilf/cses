n = int(input())

tasks = []
for _ in range(n):
    a, d = map(int, input().split())
    tasks.append((a, d))

tasks.sort()

ans = 0
t = 0
for a, d in tasks:
    t += a
    ans += d - t
print(ans)
