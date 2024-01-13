n, k = map(int, input().split())

arr = list(map(int, input().split()))

l, r = 1, sum(arr)
ans = r

max_v = max(arr)

def check(mid, arr, k) -> bool:
    cnt = 1
    sum = 0
    for v in arr:
        sum += v
        if sum > mid:
            sum = v
            cnt += 1
    # print("cnt", cnt)
    return cnt <= k


while l <= r:
    mid = (l + r) // 2
    # print("r", l, r, mid, check(mid, arr, k))
    if mid < max_v or not check(mid, arr, k):
        l = mid + 1
    else:
        r = mid - 1
        ans = min(ans, mid)
print(ans)
