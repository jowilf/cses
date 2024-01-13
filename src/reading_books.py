from collections import deque


n = int(input())

books = list(map(int, input().split()))
s = sum(books)
m = max(books)

print(2 * m if m > (s - m) else s)
