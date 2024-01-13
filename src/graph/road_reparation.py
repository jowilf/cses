n, m = map(int, input().split())
edges = []
cluster = [i for i in range(n + 1)]
size = [1 for i in range(n + 1)]


def same(u, v):
    # print("same", u, v, "->", find(u) == find(v))
    return find(u) == find(v)


def find(x):
    # if cluster[x] == x:
    #     return x
    # cluster[x] = find(cluster[x])
    while x != cluster[x]:
        x = cluster[x]
    return x


def unite(u, v):
    # print("unite", u, v)
    u, v = find(u), find(v)
    if size[u] > size[v]:
        cluster[v] = cluster[u]
        size[u] += size[v]
        size[v] = 0
    else:
        cluster[u] = cluster[v]
        size[v] += size[u]
        size[u] = 0
    # print(cluster[1:])


for i in range(m):
    edges.append(tuple(map(int, input().split())))

edges.sort(key=lambda v: v[2])

ans = 0
for u, v, wt in edges:
    if not same(u, v):
        ans += wt
        unite(u, v)

# print(cluster[1:])
# print(size[1:])
if size[find(1)] == n:
    print(ans)
else:
    print("IMPOSSIBLE")
