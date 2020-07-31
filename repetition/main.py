text = input().strip()
ans = 0
c_s = 0
c_l = ''
for l in text:
    if l == c_l:
        c_s += 1
    else:
        c_s = 1
        c_l = l
    ans = max(ans, c_s)
print(ans)