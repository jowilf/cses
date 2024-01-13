import sys

sys.setrecursionlimit(10000000)
n = int(input())

adjList = [[] for i in range(n)]

for i, boss in enumerate(map(int, input().split())):
    adjList[boss - 1].append(i + 1)

cnt = [0 for _ in range(n)]

# print(adjList)


def dfs(node):
    for emp in adjList[node]:
        cnt[node] += dfs(emp)
    cnt[node] += 1
    return cnt[node]


dfs(0)
print(" ".join(map(lambda v: str(v - 1), cnt)))
