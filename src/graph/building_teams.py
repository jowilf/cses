n, m = map(int, input().split())

adjList = [[] for i in range(n + 1)]

visited = [False for i in range(n + 1)]
color = [None for i in range(n + 1)]

for i in range(m):
    a, b = map(int, input().split())
    adjList[a].append(b)
    adjList[b].append(a)

# print(adjList)


def bfs(i):
    queue = []
    queue.append(i)
    color[i] = 0
    # print("color", color)
    while len(queue) > 0:
        node = queue.pop()
        # print(node)
        visited[node] = True
        for adj in adjList[node]:
            if not visited[adj]:
                if color[adj] is not None and ((color[node] + 1) % 2) != color[adj]:
                    print("IMPOSSIBLE")
                    exit()
                else:
                    color[adj] = (color[node] + 1) % 2
                    queue.append(adj)


for i in range(1, n + 1):
    if color[i] is None:
        bfs(i)

print(" ".join(map(lambda k: str(k + 1), color[1:])))
