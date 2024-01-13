n, t = map(int, input().split())
times = list(map(int, input().split()))


def valid(x):
    global times, t
    s = 0
    for p in times:
        s += x // p
        if s >= t:
            return True
    return s >= t


# valid = lambda x: sum([x // p for p in times]) >= t  # noqa: E731

low, high = 1, min(times) * t
while low < (high + 1):
    mid = (low + high) // 2
    # print(low, high, mid, valid(mid))
    if valid(mid):
        high = mid - 1
    else:
        low = mid + 1

print(low)
