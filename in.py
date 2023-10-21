def integerBreak(self, n: int) -> int:
    if n == 2:
        return 1
    if n == 3:
        return 2
    r = n % 3        
    return (r*(-5*r+11)+2) * (3 ** ((n+r*(3*r-7))//3))//2