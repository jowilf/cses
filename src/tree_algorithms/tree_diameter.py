import sys

sys.setrecursionlimit(10**9)
n = int(input())

adjList = [[] for _ in range(n)]

for _ in range(n - 1):
    i, j = map(int, input().split())
    i, j = i - 1, j - 1
    adjList[i].append(j)
    adjList[j].append(i)

max_depth = [None for _ in range(n)]

ans = 0


def computeMaxDepth(node, parent):
    global ans
    if max_depth[node] is not None:
        return max_depth[node]
    m1, m2 = 0, 0
    for adj in adjList[node]:
        if adj != parent:
            v = computeMaxDepth(adj, node)[0] + 1
            # print("p", adj, node ,v)
            if v >= m1:
                m1, m2 = v, m1
            elif v >= m2:
                m2 = v
    ans = max(ans, m1 + m2)
    max_depth[node] = m1, m2
    return max_depth[node]


# print(adjList)
computeMaxDepth(0, None)
# print(max_depth)
print(ans)
