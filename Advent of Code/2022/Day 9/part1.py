with open("input", "r") as f:
    data = f.readlines()

data = [x.strip() for x in data]

hx, hy = 0, 0
tx, ty = 0, 0

tall = set()
tall.add((0, 0))

def adj(hx, hy, tx, ty):
    if abs(hx - tx) <= 1 and abs(hy - ty) <= 1:
        return 0 
    if hx == tx or hy == ty:
        return 1
    return 2
    

for line in data:
    dir, dist = line[0], int(line[1:])
    if dir == "U":
        hy += dist
    elif dir == "D":
        hy -= dist
    elif dir == "L":
        hx -= dist
    elif dir == "R":
        hx += dist
    
    case = adj(hx, hy, tx, ty)

    while case != 0:
        if case == 1:
            if hx == tx:
                if hy > ty:
                    ty += 1
                else:
                    ty -= 1
            else:
                if hx > tx:
                    tx += 1
                else:
                    tx -= 1
        else:
            dx = hx - tx
            dy = hy - ty
            if dx > 0:
                tx += 1
            else:
                tx -= 1
            if dy > 0:
                ty += 1
            else:

                ty -= 1
        
        tall.add((tx, ty))
        case = adj(hx, hy, tx, ty)



print(len(tall))
