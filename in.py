def lengthOfLongestSubstring(self, s: str) -> int:
    sol = 0
    for num in range(len(s)):
        i = num
        used = []
        count = 0
        for _ in iter(int, 1):
            if s[i] in used:
                break
            elif i == len(s) - 1:
                count = count + 1
                break
            else:
                used.append(s[i])
                count = count + 1
                i = i + 1
        
        sol = max(sol, count)
    
    return sol