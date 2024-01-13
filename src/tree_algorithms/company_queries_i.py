import math

n, q = map(int, input().split())

max_k = int(math.log2(n)) + 1

ancestors = [[-1 for _ in range(max_k + 1)] for __ in range(n)]

for i, ancestor in enumerate(map(int, input().split())):
    ancestors[i + 1][0] = ancestor - 1


for k in range(1, max_k + 1):
    for i in range(n):
        previousAncestor = ancestors[i][k - 1]
        ancestorOfPreviousAncestor = (
            ancestors[previousAncestor][k - 1] if previousAncestor > -1 else -1
        )
        ancestors[i][k] = ancestorOfPreviousAncestor

# print("max_q", max_k)
# print(ancestors)
for i in range(q):
    ans, k = map(int, input().split())
    ans -= 1
    power = 0
    while k > 0:
        # print("power", ans, power, k)
        remainder = k % 2
        k = (k - remainder) // 2
        if remainder == 1:
            ans = ancestors[ans][power]
        power += 1
    print((ans + 1) if ans > -1 else ans)


# print(ancestors)

"""
10 | 2
    ---
  0 | 5  |  2
         -----
        1 | 2 ---> 01


"""
