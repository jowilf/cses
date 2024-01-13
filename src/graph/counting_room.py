from collections import deque
from sys import stdin, stdout

n, m = map(int, stdin.readline().split())

grid = [stdin.readline() for i in range(n)]

visited = [[grid[i][j] == "#" for j in range(m)] for i in range(n)]

d = ((0, 1), (1, 0), (-1, 0), (0, -1))
ans = 0
for i in range(n):
    for j in range(m):
        if grid[i][j] == "." and not visited[i][j]:
            ans += 1
            stack = []
            stack.append((i, j))
            while len(stack) > 0:
                x, y = stack.pop()
                visited[x][y] = True
                for dx, dy in d:
                    xx, yy = x + dx, y + dy
                    if (
                        xx >= 0
                        and xx < n
                        and yy >= 0
                        and yy < m
                        and not visited[xx][yy]
                    ):
                        stack.append((xx, yy))

print(ans)
