with open("input", "r") as f:
    data = f.readlines()

data = [x.strip() for x in data]

grid = [[int(x) for x in line] for line in data]

def works(grid, i, j):
    if i == 0 or i == len(grid)-1 or j == 0 or j == len(grid[i])-1:
        return True
    
    val = grid[i][j]
    up = 0
    for k in range(i-1, -1, -1):
        up += 1
        if grid[k][j] >= val:
            break
    
    down = 0
    for k in range(i+1, len(grid)):
        down += 1
        if grid[k][j] >= val:
            break
    
    left = 0
    for k in range(j-1, -1, -1):
        left += 1
        if grid[i][k] >= val:
            break
    
    right = 0
    for k in range(j+1, len(grid[i])):
        right += 1
        if grid[i][k] >= val:
            break

    
    return up * down * left * right
            



c = 0
for i in range(len(grid)):
    for j in range(len(grid[i])):
        c = max(c, works(grid, i, j))

        
print(c)