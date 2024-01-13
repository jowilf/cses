n = int(input())
arr = map(int, input().split())

d = dict()
current_count = 0
best_count = 0
start_pos = 0
for i, value in enumerate(arr):
    k = d.get(value, None)
    if k is None or k < start_pos:
        current_count += 1
        d[value] = i
    else:
        start_pos = k + 1
        current_count = i - start_pos + 1
        d[value] = i
    best_count = max(best_count, current_count)
    # print(f"i={i}; cc={current_count}, bc={best_count}, sp={start_pos}")
print(best_count)
