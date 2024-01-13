n = int(input())
arr = [int(x) for x in input().split()]

idx_arr = [0 for _ in range(n + 1)]
for i, v in enumerate(arr):
    idx_arr[v] = i

ans = 0
idx = -1
for i in range(1, n + 1):
    n_idx = idx_arr[i]
    if n_idx < idx:
        ans += 1
    idx = n_idx
    # print(i, idx, ans)
print(ans + 1)
