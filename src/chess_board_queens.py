cnt = 0


def search(stack, x2):
    global cnt, grid
    if len(stack) == 8:
        cnt += 1
        # print([(x, y, grid[y][x]) for (x, y) in stack])
        return
    for y2 in range(8):
        good = True
        for x1, y1 in stack:
            if x1 == x2 or y1 == y2 or abs(x1 - x2) == abs(y1 - y2):
                good = False
        if good and grid[y2][x2] != "*":
            search(stack + [(x2, y2)], x2 + 1)


grid = [list(input().strip()) for i in range(8)]
search([], 0)
print(cnt)
