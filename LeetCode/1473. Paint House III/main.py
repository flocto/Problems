from functools import lru_cache

class Solution:
    def minCost(self, houses: list[int], cost: list[list[int]], m: int, n: int, target: int) -> int:
        @lru_cache(None) # dp
        def dp(i, k, prev):
            # minimum cost to paint houses i <= m with k neighborhoods, where house i - 1 is color prev
            if i == len(houses):
                if k == 0:
                    return 0 # dont need to do anything
                return float('inf')
            res = 10000000
            if houses[i] == 0: # have to paint
                for j in range(1, n + 1):
                    res = min(res, dp(i + 1, k - (j != prev), j) + cost[i][j - 1])
            else: # dont have to paint
                res = dp(i + 1, k - (houses[i] != prev), houses[i])
            return res
        
        ret = dp(0, target, 0)
        return ret if ret < 10000000 else -1