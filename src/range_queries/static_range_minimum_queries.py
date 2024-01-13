class SegmentTree:
    def __init__(self, arr) -> None:
        self.arr = arr
        self.n = len(arr)
        self.tree = [None for _ in range(self.n * 4)]
        self.build(0, 0, self.n - 1)

    def left(self, idx):
        return 2 * idx + 1

    def right(self, idx):
        return 2 * idx + 2

    def build(self, idx, l, r):
        # print("build", idx, l, r)
        if l == r:
            self.tree[idx] = l
            # print(self.tree)
        else:
            mid = (l + r) // 2
            idx1 = self.left(idx)
            idx2 = self.right(idx)
            self.build(idx1, l, mid)
            self.build(idx2, mid + 1, r)
            p1 = self.arr[self.tree[idx1]]
            p2 = self.arr[self.tree[idx2]]
            self.tree[idx] = self.tree[idx1] if p1 < p2 else self.tree[idx2]

    def rmq(self, l, r):
        return self.arr[self.rmiq(0, 0, self.n - 1, l, r)]

    def _rmq(self, idx, l, r, ql, qr):
        if r < ql or l > qr:
            return -1
        if l >= ql and r <= qr:
            return self.tree[idx]
        mid = (l + r) // 2
        p1 = self.rmq(self.left(idx), l, mid, ql, qr)
        p2 = self.rmq(self.right(idx), mid + 1, r, ql, qr)
        if p1 == -1:
            return p2
        if p2 == -1:
            return p1
        return p1 if self.arr[p1] < self.arr[p2] else p2


n, q = map(int, input().split())
arr = list(map(int, input().split()))  # list(map(int, "3 2 4 5 1 1 5 3".split()))
s = SegmentTree(arr)
# print(s.tree)

for _ in range(q):
    i, j = map(int, input().split())
    print(s.rmq(i - 1, j - 1))

# print(s.rmq(0, 3))
