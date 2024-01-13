class mylist(list):
    def _init__(self, *args, **kwargs):
        super.__init__(*args, **kwargs)
        self._id = 0

    def __repr__(self) -> str:
        return "(%d: %s)" % (self._id, super().__repr__())


n = int(input())
left = mylist(range(1, n + 1))
middle = mylist()
right = mylist()

left._id = 1
middle._id = 2
right._id = 3
solution = []


def move(source, target, auxiliary, n):
    global solution
    # print(source, target, auxiliary, n)
    if n > 0:
        move(source, auxiliary, target, n - 1)
        target.append(source.pop())
        solution.append((source._id, target._id))
        move(auxiliary, target, source, n - 1)


move(left, right, middle, n)
print(len(solution))
for i, j in solution:
    print(i, j)
