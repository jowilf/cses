class Solution:
    def containsNearbyDuplicate(self, nums: list[int], k: int) -> bool:
        v = list(map(lambda i: (i[1], i[0]), enumerate(nums)))
        v = sorted(v, key=lambda i: (i[0], i[1]))
        print(v)
        for i in range(len(v)):
            if i > 0 and v[i - 1][0] == v[i][0] and (v[i][1] - v[i - 1][1]) <= k:
                return True
        return False


print(Solution().containsNearbyDuplicate([1, 2, 3, 1, 2, 3], 2))
