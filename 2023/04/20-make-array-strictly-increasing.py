from typing import List, Optional
from functools import cache
import sys, bisect

class Solution:
    def makeArrayIncreasing(self, arr1: List[int], arr2: List[int]) -> int:
        arr2.sort()
        ma = float('inf')
        @cache
        def f(i,pre):
            if i==len(arr1):return 0
            res = ma
            pos = bisect.bisect_right(arr2, pre)
            if pos < len(arr2):
                res = f(i+1, arr2[pos])+1
            if arr1[i] > pre:
                res = min(res, f(i+1, arr1[i]))
            return res
        ans = f(0,-1)
        return ans if ans != ma else -1
