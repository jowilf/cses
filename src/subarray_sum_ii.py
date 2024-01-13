n, x = map(int, input().split())

d = {}
ans = 0
acc = 0
for v in map(int, input().split()):
    acc += v
    if acc == x:
        ans += 1
    ans += d.get(acc - x, 0)
    d[acc] = d.get(acc, 0) + 1
print(ans)
