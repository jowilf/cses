arr = []
for i in range(int(input())):
    a, b = map(int, input().split())
    arr.extend([(a, 1), (b, -1)])

arr.sort(key=lambda k: k[0])
ans, c = 0, 0
for v in arr:
    c += v[1]
    ans = max(ans, c)
print(ans)
