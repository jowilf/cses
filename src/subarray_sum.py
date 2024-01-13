n, x = map(int, input().split())

arr = list(map(int, input().split()))

ans = current_sum = l = r = 0
while r < n:
    # print("c", current_sum, l, r)
    v = arr[r]
    if v > x:
        l = r = r + 1
        current_sum = 0
        ans += 1 if v == x else 0
    elif (current_sum + v) > x:
        current_sum -= arr[l]
        l += 1
    else:
        current_sum += v
        r += 1
        if current_sum == x:
            ans += 1
print(ans)
