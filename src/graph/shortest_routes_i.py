import heapq


n, m = map(int, input().split())

adjList = [[] for _ in range(n + 1)]

for i in range(m):
    a, b, c = map(int, input().split())
    adjList[a].append((b, c))

d = [float("inf") for i in range(n + 1)]
d[1] = 0

visited = [False] * (n + 1)

queue = []
heapq.heappush(queue, (0, 1))

while queue:
    dist, node = heapq.heappop(queue)

    if visited[node]:
        continue

    visited[node] = True

    for adj, wt in adjList[node]:
        if dist + wt < d[adj]:
            d[adj] = dist + wt
            heapq.heappush(queue, (d[adj], adj))

print(" ".join(map(str, d[1:])))
