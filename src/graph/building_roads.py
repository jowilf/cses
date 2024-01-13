n, m = map(int, input().split())

adjList = [[] for i in range(n + 1)]

visited = [False for i in range(n + 1)]

for i in range(m):
    a, b = map(int, input().split())
    adjList[a].append(b)
    adjList[b].append(a)


def bfs(i):
    queue = []
    queue.append(i)
    while len(queue) > 0:
        node = queue.pop()
        visited[node] = True
        for adj in adjList[node]:
            if not visited[adj]:
                queue.append(adj)


ans = []
for i in range(1, n + 1):
    if not visited[i]:
        ans.append(i)
        bfs(i)
print(len(ans) - 1)
if len(ans) > 1:
    for i in range(1, len(ans)):
        print(ans[i - 1], ans[i])
