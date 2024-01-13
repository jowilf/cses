movies = []
for i in range(int(input())):
    a, b = map(int, input().split())
    movies.append((a, b))
movies.sort(key=lambda v: v[1])
lastEndTime = -1
ans, i = 0, 0
while i < len(movies):
    if movies[i][0] >= lastEndTime:
        ans += 1
        lastEndTime = movies[i][1]
    i += 1
print(ans)
