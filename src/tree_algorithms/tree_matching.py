import sys

sys.setrecursionlimit(10000000)
n = int(input())

adjList = [[] for i in range(n + 1)]

for i in range(n - 1):
    a, b = map(int, input().split())
    adjList[a].append(b)
    adjList[b].append(a)

used = [0 for _ in range(n + 1)]

# print(adjList)

ans = 0


def maxMatching(node, parent):
    # print("ma", node, parent)
    global ans
    for child in adjList[node]:
        if child != parent:
            maxMatching(child, node)

    if not used[node] and not used[parent] and parent != 0:
        used[node] = used[parent] = 1
        ans += 1


# print(adjList)
# print(used)


maxMatching(1, 0)
print(ans)
# print(" ".join(map(lambda v: str(v - 1), dp)))
