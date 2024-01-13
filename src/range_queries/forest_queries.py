n, q = map(int, input().split())

grid = [list(map(lambda v: 1 if v == "*" else 0, input())) for _ in range(n)]

acc = [[0 for _ in range(n)] for _ in range(n)]

for i in range(n):
    for j in range(n):
        acc[i][j] += grid[i][j]
        if i - 1 >= 0:
            acc[i][j] += acc[i - 1][j]
        if j - 1 >= 0:
            acc[i][j] += acc[i][j - 1]
        if i - 1 >= 0 and j - 1 >= 0:
            acc[i][j] -= acc[i - 1][j - 1]

# for v in grid:
#     print(v)
# print("-----")
# for v in acc:
#     print(v)
for _ in range(q):
    i1, j1, i2, j2 = map(lambda v: int(v) - 1, input().split())
    ans = acc[i2][j2]
    if j1 - 1 >= 0:
        ans -= acc[i2][j1 - 1]
    if i1 - 1 >= 0:
        ans -= acc[i1 - 1][j2]
    if i1 - 1 >= 0 and j1 - 1 >= 0:
        ans += acc[i1 - 1][j1 - 1]
    print(ans)
