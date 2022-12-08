with open("input", "r") as f:
    data = f.readlines()

data = [x.strip() for x in data]

grid = [[int(x) for x in line] for line in data]

def works(grid, i, j):
    if i == 0 or i == len(grid)-1 or j == 0 or j == len(grid[i])-1:
        return True
    
    val = grid[i][j]
    up = True
    for k in range(i-1, -1, -1):
        if grid[k][j] >= val:
            up = False
            break
    
    down = True
    for k in range(i+1, len(grid)):
        if grid[k][j] >= val:
            down = False
            break
    
    left = True
    for k in range(j-1, -1, -1):
        if grid[i][k] >= val:
            left = False
            break
    
    right = True
    for k in range(j+1, len(grid[i])):
        if grid[i][k] >= val:
            right = False
            break
    
    return up or down or left or right
            



c = 0
for i in range(len(grid)):
    for j in range(len(grid[i])):
        if works(grid, i, j):
            c += 1
        
print(c)