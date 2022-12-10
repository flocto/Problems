with open("input", "r") as f:
    data = f.readlines()

data = [x.strip() for x in data]

X = 1
cycle = 1

marker = 20

screen = ["" for _ in range(6)]

def draw():
    i = (cycle - 1) // 40 # row 2 starts at cycle 41
    if abs(X - len(screen[i])) < 2: # sprite is 3 wide
        screen[i] += "#"
    else:
        screen[i] += "."


for line in data:
    if line != "noop":
        dx = int(line.split(" ")[1])
        draw()
        cycle += 1
        draw()
        cycle += 1
        X += dx
    else:
        draw()
        cycle += 1


        
for line in screen:
    print(line)