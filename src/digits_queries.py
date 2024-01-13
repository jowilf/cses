for _ in range(int(input())):
    n, k = int(input()), 1
    while n > (c := 9 * k * 10 ** (k - 1)):
        n -= c
        k += 1
    n -= 1
    number = str(10 ** (k - 1) + n // k)
    print(number[n % k])
