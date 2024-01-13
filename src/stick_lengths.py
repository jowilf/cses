n = int(input())
arr = list(map(int, input().split()))
arr.sort()
medians = []
if n % 2 != 0:
    medians = [arr[n // 2]]
else:
    k = n // 2
    median = arr[k - 1] + arr[k]
    if median % 2 == 0:
        medians = [median // 2]
    else:
        medians = [median // 2, median // 2 + 1]
ans = None
for v in medians:
    diff = 0
    for it in arr:
        diff += abs(v - it)
    if ans is None or diff < ans:
        ans = diff
print(ans)
