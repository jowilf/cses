from typing import List
import heapq


class Solution:
    def kSmallestPairs(
        self, nums1: List[int], nums2: List[int], k: int
    ) -> List[List[int]]:
        ans = []
        q = []
        for i in range(min(k, len(nums1))):
            heapq.heappush(q, (nums1[i] + nums2[0], i, 0))

        while k > 0 and len(q) > 0:
            _s, i, j = heapq.heappop(q)
            if j + 1 < len(nums2):
                heapq.heappush(q, (nums1[i] + nums2[j + 1], i, j + 1))
            k -= 1
            ans.append([nums1[i], nums2[j]])
        return ans


print(
    Solution().kSmallestPairs(
        [1, 2, 3, 3, 3],
        [-3, 22, 35, 56, 70, 100, 123, 200],
        10,
    )
)
# print(Solution().kSmallestPairs([1, 2, 4], [-1, 1, 2], 100))

"""
[1, 2, 4]
[-1, 1, 2]
"""
