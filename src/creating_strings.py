import itertools


s = input().strip()
ans = sorted(set(itertools.permutations(s)))
print(len(ans))
for v in ans:
    print("".join(v))
