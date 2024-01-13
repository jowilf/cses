n, x = map(int, input().split())
arr = map(int, input().split())
d = dict()
for i, v in enumerate(arr):
    k = d.get(x - v, -1)
    if k >= 0:
        print(i + 1, k)
        exit()
    d[v] = i + 1
print("IMPOSSIBLE")
