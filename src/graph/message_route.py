n, m = map(int, input().split())

adjList = [[] for i in range(n + 1)]

visited = [False for i in range(n + 1)]
predecessor = [-1 for i in range(n + 1)]
for i in range(m):
    a, b = map(int, input().split())
    adjList[a].append(b)
    adjList[b].append(a)


def printOut():
    path = [n]
    i = predecessor[n]
    while i != -1:
        path.append(i)
        i = predecessor[i]
    print(len(path))
    print(" ".join(map(str, reversed(path))))


def bfs(i):
    queue = []
    queue.append(i)
    visited[i] = True
    while len(queue) > 0:
        node = queue.pop(0)
        # print(queue, node)
        for adj in adjList[node]:
            if not visited[adj]:
                # print(predecessor)
                queue.append(adj)
                predecessor[adj] = node
                visited[adj] = True
            if adj == n:
                printOut()
                exit()


# print(adjList)
bfs(1)
print("IMPOSSIBLE")
