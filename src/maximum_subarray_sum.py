input()

arr = list(map(int, input().split()))
if max(arr) < 0:
    print(max(arr))
else:
    current_sum = 0
    best_sum = 0
    for v in arr:
        current_sum = max(0, current_sum + v)
        best_sum = max(best_sum, current_sum)
    print(best_sum)
