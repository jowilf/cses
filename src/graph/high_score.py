import sys

sys.setrecursionlimit(10000)
n, m = map(int, input().split())

revAdjList = [[] for _ in range(n + 1)]
visited = [False for _ in range(n + 1)]

edges = []
d = [float("-inf") for i in range(n + 1)]

for i in range(m):
    a, b, c = map(int, input().split())
    # adjList[a].append((b, c))
    revAdjList[b].append((a, c))
    edges.append((a, b, c))


def dfs(node):
    visited[node] = True
    for adj, wt in revAdjList[node]:
        if not visited[adj]:
            dfs(adj)


dfs(n)
# print(visited)
d[1] = 0
for i in range(n):
    for a, b, c in edges:
        if d[a] + c > d[b]:
            d[b] = d[a] + c
            if i == n - 1 and (visited[a] or visited[b]):
                # print("cycle", a, b)
                print(-1)
                exit(0)
# print(d)
print(d[n])

"""
5 5
2 0 1
3 0 -1
0 4 4
4 1 2
4 2 -1
"""
