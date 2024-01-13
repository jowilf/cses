n, x = map(int, input().split())
arr = list(map(int, input().split()))

map = {}
l = r = 0
ans = 0

while r < n:
    map[arr[r]] = map.get(arr[r], 0) + 1
    while len(map) > x:
        map[arr[l]] = map.get(arr[l]) - 1
        if map[arr[l]] == 0:
            del map[arr[l]]
        l += 1
    ans += r - l + 1
    r += 1
print(ans)
