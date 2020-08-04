k,D=int(input()),"YES"
for i in range(k):
        a,b=map(int,input().split())
        if a==0 or b==0:print(D if a==b else'NO')
        else:print(D if(a+b)%3==0 and max(a,b)/min(a,b)<=2 else'NO')