import sys


def input():
    return sys.stdin.readline()


def print(x):
    return sys.stdout.write(str(x) + " ")


sys.setrecursionlimit(10**9)
n = int(input())

adjList = [[] for _ in range(n)]

for _ in range(n - 1):
    i, j = map(int, input().split())
    i, j = i - 1, j - 1
    adjList[i].append(j)
    adjList[j].append(i)

dist = [None for _ in range(n)]
ans = [None for _ in range(n)]


def dfs1(node, parent):
    _sum, cnt = 0, 0
    for adj in adjList[node]:
        if adj != parent:
            s, c = dfs1(adj, node)
            _sum += s + c + 1
            cnt += c + 1
    dist[node] = _sum, cnt
    return dist[node]


def dfs2(node, parent):
    if parent is not None:
        s, c = dist[node]
        # print("bk: ", node + 1, ans[parent], (s + c + 1), (n - c - 1))
        ans[node] = s + ans[parent] - (s + c + 1) + (n - c - 1)
    else:
        ans[node] = dist[node][0]
    for adj in adjList[node]:
        if adj != parent:
            dfs2(adj, node)


dfs1(0, None)
dfs2(0, None)

# print(dist)

# print(max_depth1)
print(" ".join(map(str, ans)))
