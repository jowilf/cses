n,k,ans=int(input()),1,0
while 5**k<=n:
    ans+=int(n/(5**k))
    k+=1
print(ans)