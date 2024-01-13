from sys import stdin
from collections import deque

n, m = map(int, stdin.readline().split())

grid = [stdin.readline() for i in range(n)]

visited = [[False for j in range(m)] for i in range(n)]
monster_visited = [[False for j in range(m)] for i in range(n)]

monsters_queue = deque()
queue = deque()
dist = [[0 for j in range(m)] for i in range(n)]
monsters_dist = [[None for j in range(m)] for i in range(n)]
predecessor = [[None for j in range(m)] for i in range(n)]

for i in range(n):
    for j in range(m):
        v = grid[i][j]
        if v == "#":
            visited[i][j] = True
            monster_visited[i][j] = True
        elif v == "M":
            visited[i][j] = True
            monsters_queue.append((i, j))
            monster_visited[i][j] = True
            monsters_dist[i][j] = 0
        elif v == "A":
            queue.append((i, j))
            visited[i][j] = True

d = ((0, 1, "R"), (1, 0, "D"), (-1, 0, "U"), (0, -1, "L"))


# Monster bfs
while len(monsters_queue) > 0:
    x, y = monsters_queue.popleft()
    for dx, dy, dir in d:
        xx, yy = x + dx, y + dy
        if xx >= 0 and xx < n and yy >= 0 and yy < m and not monster_visited[xx][yy]:
            # print("monster", xx, yy, monsters_dist[x][y] + 1)
            monsters_dist[xx][yy] = monsters_dist[x][y] + 1
            monsters_queue.append((xx, yy))
            monster_visited[xx][yy] = True

# Human bfs
while len(queue) > 0:
    x, y = queue.popleft()
    # print("pop", x, y)
    # Reach bondary
    if x == 0 or x == n - 1 or y == 0 or y == m - 1:
        print("YES")
        # print(predecessor)
        paths = []
        a, b, dir = x, y, None
        while predecessor[a][b] is not None:
            a, b, dir = predecessor[a][b]
            paths.append(dir)
        print(len(paths))
        print("".join(reversed(paths)))
        # print(monsters_dist)
        exit()

    for dx, dy, dir in d:
        xx, yy = x + dx, y + dy
        # print(xx,yy)
        if (
            xx >= 0
            and xx < n
            and yy >= 0
            and yy < m
            and not visited[xx][yy]
            and (
                monsters_dist[xx][yy] is None
                or (dist[x][y] + 1) < monsters_dist[xx][yy]
            )
        ):
            dist[xx][yy] = dist[x][y] + 1
            queue.append((xx, yy))
            visited[xx][yy] = True
            predecessor[xx][yy] = (x, y, dir)

            # print("reach", xx, yy)
# print(monsters_dist)
# print(dist)
print("NO")
