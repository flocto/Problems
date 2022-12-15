with open("input", "r") as f:
    data = f.readlines()

data = [x.strip() for x in data]

lines = []
for line in data:
    line = line.split(" -> ")
    lines.append([eval(x) for x in line])

maxx, maxy = 0, 0
minx, miny = 500, 500
for line in lines:
    for x, y in line:
        maxx = max(maxx, x)
        maxy = max(maxy, y)
        minx = min(minx, x)
        miny = min(miny, y)

grid = [[0 for x in range(maxx+1 - minx)] for y in range(maxy+1)]

def draw_line(line):
    x1, y1 = line[0]
    x2, y2 = line[1]


    if x1 == x2:
        if y1 > y2:
            y1, y2 = y2, y1
        for y in range(y1, y2+1):
            grid[y][x1-minx] = 1
    else:
        if x1 > x2:
            x1, x2 = x2, x1
        for x in range(x1, x2+1):
            grid[y1][x-minx] = 1

for section in lines:
    for i in range(len(section)-1):
        draw_line([section[i], section[i+1]])

def print_grid():
    for row in grid:
        for x in row:
            if x == 0:
                print(".", end="")
            elif x == 1:
                print("#", end="")
            elif x == 2:
                print("o", end="")

        print()
    print()
print_grid()

# grid finish, begin sim
abyss = False


def move(x, y):
    if y == len(grid) - 1: # nothing below
        global abyss
        abyss = True
        return

    if grid[y+1][x] == 0:
        return move(x, y+1)
    elif grid[y+1][x-1] == 0:
        return move(x-1, y+1)
    elif grid[y+1][x+1] == 0:
        return move(x+1, y+1)
    else:
        grid[y][x] = 2
        return

count = 0
while not abyss:
    move(500-minx, 0)
    if abyss:
        break
    count += 1

print_grid()
print(count)