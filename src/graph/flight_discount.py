import heapq


n, m = map(int, input().split())

adjList = [[] for _ in range(n + 1)]
revAdjList = [[] for _ in range(n + 1)]

for i in range(m):
    a, b, c = map(int, input().split())
    adjList[a].append((b, c))
    revAdjList[b].append((a, c))


def dijskra(i, j, adjList):
    d = [float("inf") for _ in range(n + 1)]
    d[i] = 0

    visited = [False] * (n + 1)

    queue = []
    heapq.heappush(queue, (0, 0, i))

    while queue:
        dist, _max, node = heapq.heappop(queue)
        # print("dist", d)
        # print("pop", dist, _max, node)

        if visited[node]:
            continue

        visited[node] = True

        if node == j:
            return d[j]

        for adj, wt in adjList[node]:
            if wt > _max:
                _new_dist = dist + (_max - _max // 2) + wt // 2
            else:
                _new_dist = dist + wt
            # print("adj", adj, node, _new_dist, d[adj])
            if _new_dist <= d[adj]:
                d[adj] = _new_dist
                # print("push", (d[adj], max(_max, wt), adj))
                heapq.heappush(queue, (d[adj], max(_max, wt), adj))


# print(adjList)

print(min(dijskra(1, n, adjList), dijskra(n, 1, revAdjList)))
