def generate(tmp: str, l: int, r: int, n: int):
    if l == r == n:
        print(tmp)

    if l < n:
        generate(tmp + "(", l + 1, r, n)
    if l > r:
        generate(tmp + ")", l, r + 1, n)


generate("", 0, 0, 4)
