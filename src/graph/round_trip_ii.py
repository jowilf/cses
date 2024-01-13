n, m = map(int, input().split())

adjList = [[] for i in range(n + 1)]

visited = [False for i in range(n + 1)]
predecessor = [-1 for i in range(n + 1)]
dist = [0 for i in range(n + 1)]
for i in range(m):
    a, b = map(int, input().split())
    adjList[a].append(b)
    # adjList[b].append(a)


def printOut():
    path = [n]
    i = predecessor[n]
    while i != -1:
        path.append(i)
        i = predecessor[i]
    print(len(path))
    print(" ".join(map(str, path)))


def bfs(i):
    print("bfs", i)
    queue = []
    queue.append(i)
    dist[i] = 0
    while len(queue) > 0:
        node = queue.pop()
        print(queue, node)
        for adj in adjList[node]:
            if not visited[adj]:
                # print(predecessor)
                queue.append(adj)
                predecessor[adj] = node
                dist[adj] = dist[node] + 1
                visited[node] = True
            else:
                # paths = [adj]
                # k = adj
                # while predecessor[k] != -1 and predecessor[k] != adj:
                #     k = predecessor[k]
                #     paths.append(k)
                paths2 = [node]
                k = node
                while predecessor[k] != -1 and k != adj:
                    k = predecessor[k]
                    paths2.append(k)
                paths = [adj]
                if paths2[-1] != adj:
                    k = adj
                    while predecessor[k] != -1:
                        k = predecessor[k]
                        paths.append(k)
                # print(paths, paths2)
                print(len(paths) + len(paths2))
                print(" ".join(map(str, reversed(paths + paths2))))
                exit()


# print(adjList)
for i in range(1, n + 1):
    if not visited[i]:
        bfs(i)
print("IMPOSSIBLE")
