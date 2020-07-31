n = int(input())
s = (n * (n+1))/2
for i in input().split():
    s-=int(i)
print(int(s))
