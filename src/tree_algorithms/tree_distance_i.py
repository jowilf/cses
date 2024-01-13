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

max_depth1 = [None for _ in range(n)]
max_depth2 = [None for _ in range(n)]

ans = 0


def dfs1(node, parent):
    d1, d2 = 0, 0
    for adj in adjList[node]:
        if adj != parent:
            v = dfs1(adj, node)[0] + 1
            if v >= d1:
                d1, d2 = v, d1
            elif v > d2:
                d2 = v
    max_depth1[node] = d1, d2
    # print(node + 1, parent + 1, d)
    return max_depth1[node]


def dfs2(node, parent):
    parentMaxL = -1
    if parent is not None:
        if max_depth1[node][0] == max_depth1[parent][0] - 1:
            parentMaxL = max_depth1[parent][1] + 1
        else:
            parentMaxL = max_depth1[parent][0] + 1
    if parentMaxL >= max_depth1[node][0]:
        max_depth1[node] = parentMaxL, max_depth1[node][0]
    elif parentMaxL >= max_depth1[node][1]:
        max_depth1[node] = max_depth1[node][0], parentMaxL

    max_depth2[node] = max_depth1[node][0]
    # print(node, max_depth1)
    for adj in adjList[node]:
        if adj != parent:
            dfs2(adj, node)


dfs1(0, None)
dfs2(0, None)

# print(max_depth1)
print(" ".join(map(str, max_depth2)))
