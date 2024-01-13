n, q = map(int, input().split())
arr = list(map(int, input().split()))
acc = [arr[0]]
for i in range(1, n):
    acc.append(acc[i - 1] + arr[i])

for i in range(q):
    a, b = map(int, input().split())
    if a == 1:
        print(acc[b - 1])
    else:
        print(acc[b - 1] - acc[a - 2])
