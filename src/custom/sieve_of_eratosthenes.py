# Sum of prime numbers between 1 & 2.000.000

M = 2 * 10**6
sieveOfErastosthenes_array = [True for _ in range(M + 2)]
cursor = 2

# 1 2 3 4 5 6 7 8 9 10
# 2 <-> 4, 6, 8, 10
# 3 <-> 6, 9

while cursor < M:
    i = 2
    while cursor * i <= M:
        sieveOfErastosthenes_array[cursor * i] = False
        i += 1
    cursor += 1
    while not sieveOfErastosthenes_array[cursor]:
        cursor += 1

_sum = 0
for i in range(1, M + 1):
    _sum += i if sieveOfErastosthenes_array[i] else 0

print(_sum)