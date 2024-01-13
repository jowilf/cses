n = int(input())
arr = list(map(int, input().split()))
# arr = list(map(int, "1 2 3 4 5 6 7 8 9 10".split()))
# n = len(arr)

A = [-1 for _ in range(n + 1)]
# P = [-1 for _ in range(n + 1)]
A[0] = 0
last = 0
# print(A)
for i in range(n):
    # Binary Search
    l, r = 0, last
    while r >= l:
        mid = (l + r) // 2
        if arr[A[mid]] < arr[i]:
            l = mid + 1
        else:
            r = mid - 1
    A[l] = i
    # P[i] = A[l - 1]
    if l > last:
        last += 1
    # print("A:", A)
    # print("P:", P)
print(last + 1)
# arr2 = []
# i = A[last]
# while i != -1:
#     arr2.append(arr[i])
#     i = P[i]
# print(arr2, last)
