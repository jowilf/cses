n, m, q = map(int, input().split())

dist = [[(0 if i == j else float("inf")) for i in range(n)] for j in range(n)]

for i in range(m):
    a, b, c = map(int, input().split())
    dist[a - 1][b - 1] = min(c, dist[a - 1][b - 1])
    dist[b - 1][a - 1] = min(c, dist[b - 1][a - 1])

# print(dist)
for k in range(n):
    for i in range(n):
        for j in range(n):
            dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])

# print(dist)
for i in range(q):
    a, b = map(int, input().split())
    print(-1 if dist[a - 1][b - 1] == float("inf") else dist[a - 1][b - 1])
