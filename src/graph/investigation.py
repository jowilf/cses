import heapq

MOD = 10**9 + 7
n, m = map(int, input().split())

adjList = [[] for _ in range(n + 1)]

for i in range(m):
    a, b, c = map(int, input().split())
    adjList[a].append((b, c))

d = [float("inf") for i in range(n + 1)]
d2 = [0 for i in range(n + 1)]
d3 = [0 for i in range(n + 1)]
d4 = [0 for i in range(n + 1)]
d[1] = 0
d2[1] = 1

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
            d2[adj] = d2[node]
            d3[adj] = d3[node] + 1
            d4[adj] = d4[node] + 1
            heapq.heappush(queue, (d[adj], adj))
        elif dist + wt == d[adj]:
            # print("eq", node, adj, d[adj], sep=" -- ")
            d2[adj] += d2[node]
            d2[adj] %= MOD
            d3[adj] = min(d3[adj], d3[node] + 1)
            d4[adj] = max(d4[adj], d4[node] + 1)
# print(d, d2, d3, d4)
print(d[n], d2[n], d3[n], d4[n])
"""
4 5
1 4 5
1 2 4
2 4 5
1 3 2
3 4 3
"""
