def generate_gray_code(n: int):
    if n == 1:
        return ["0", "1"]
    else:
        g = generate_gray_code(n - 1)
        return [("0" + v) for v in g] + [("1" + v) for v in reversed(g)]


print("\n".join(generate_gray_code(int(input()))))
