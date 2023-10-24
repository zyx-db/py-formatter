def threeSum(self, nums: List[int]) -> List[List[int]]:
    n = len(nums)
    nums.sort()
    sol = set()

    for i in range(n-2):
        j = i+1
        k = n-1
        while j < k:
            if nums[i] + nums[j] + nums[k] < 0:
                j += 1
            elif nums[i] + nums[j] + nums[k] > 0:
                k -= 1
            else:
                sol.add((nums[i], nums[j], nums[k]))
                j += 1
                k -= 1
    
    return list(sol)