n, m = map(int, input().split())
values = [-1] + [int(x) for x in input().split()]

positionOf = [0 for _ in range(n + 1)]
for i, v in enumerate(values):
    positionOf[v] = i

ans = 1
previous_idx = -1
# print(indexes[1:])
for i in range(1, n + 1):
    if positionOf[i] < previous_idx:
        ans += 1
    previous_idx = positionOf[i]
# print("raw", ans)
out = ""
for _ in range(m):
    l, r = map(int, input().split())
    a, b = values[l], values[r]
    affected = set()
    for x in [a, b]:
        if x - 1 >= 1:
            affected.add((x - 1, x))
        if x + 1 <= n:
            affected.add((x, x + 1))
    print(affected)
    for i, j in affected:
        if positionOf[i] > positionOf[j]:
            ans -= 1
    values[l], values[r] = b, a
    positionOf[a], positionOf[b] = r, l
    for i, j in affected:
        if positionOf[i] > positionOf[j]:
            ans += 1
    out += str(ans) + "\n"
print(out)
