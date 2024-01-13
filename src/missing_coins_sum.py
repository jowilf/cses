input()
arr = [int(x) for x in input().split()]
arr.sort()

current_sum = 0
for x in arr:
    if x > (current_sum + 1):
        print(current_sum + 1)
        exit(0)
    current_sum += x
print(current_sum + 1)
