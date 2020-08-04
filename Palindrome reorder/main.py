text = input().strip()
#text = open('in').readline().strip()
keys = set()
count = dict()
for l in text:
    count[l] = count.get(l, 0)+1
    keys.add(l)
ic = 0
c1 = ''
for k in keys:
    if count[k] % 2 != 0:
        ic += 1
        c1 = k
if ic > 1:
    print("NO SOLUTION")  # AAAACACABBB
else:
    i, l = 0, len(text)-1
    text = list(text)
    #print(count, ic, c1)
    for k in keys:
        if (count[k] // 2) > 0:
            for j in range(count[k]//2):
                #print('->', (i, j, k))
                text[i] = k
                text[l] = k
                i += 1
                l -= 1
    if c1 != '':
        text[i] = c1
    print("".join(text))
