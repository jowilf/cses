from collections import deque

n = int(input())

arr = deque(range(n))
ans = []
while len(arr) > 0:
    # print(arr)
    arr.append(arr.popleft())
    ans.append(str(arr.popleft() + 1))
print(" ".join(ans))
