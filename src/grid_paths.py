# path = input()
path = input()
assert len(path) == 48
N = 7
grid = [[False for __ in range(N)] for _ in range(N)]


def search(x, y, idx):
    global grid
    if (x, y) == (0, N - 1):
        if idx == 48:
            return 1
        return 0
    if idx == 48:
        return 0
    if (
        ((x + 1) == N or grid[x + 1][y])
        and ((x - 1) < 0 or grid[x - 1][y])
        and (0 <= y - 1 < N and not grid[x][y - 1])
        and (0 <= y + 1 < N and not grid[x][y + 1])
    ):
        return 0
    if (
        ((y + 1) == N or grid[x][y + 1])
        and ((y - 1) < 0 or grid[x][y - 1])
        and (0 <= x - 1 < N and not grid[x - 1][y])
        and (0 <= x + 1 < N and not grid[x + 1][y])
    ):
        return 0
    d = ["D", "U", "L", "R"]
    dxy = [(0, 1), (0, -1), (-1, 0), (1, 0)]
    ans = 0
    for i, (dx, dy) in enumerate(dxy):
        if path[idx] == d[i] or path[idx] == "?":
            nx, ny = x + dx, y + dy
            if 0 <= nx < N and 0 <= ny < N and not grid[nx][ny]:
                grid[x][y] = True
                ans += search(nx, ny, idx + 1)
                grid[x][y] = False
    return ans


print(search(0, 0, 0))
