with open("input", "r") as f:
    data = f.readlines()

data = [x.strip() for x in data]

ropes = [(0, 0) for _ in range(10)] # 10 knots

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
    for step in range(dist):
        if dir == "U":
            ropes[0] = (ropes[0][0], ropes[0][1] + 1)
        elif dir == "D":
            ropes[0] = (ropes[0][0], ropes[0][1] - 1)
        elif dir == "L":
            ropes[0] = (ropes[0][0] - 1, ropes[0][1])
        elif dir == "R":
            ropes[0] = (ropes[0][0] + 1, ropes[0][1])

        for i in range(1, 10):
            hx, hy = ropes[i-1]
            tx, ty = ropes[i]
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

                if i == 9:
                    tall.add((tx, ty))

                case = adj(hx, hy, tx, ty)
            ropes[i] = (tx, ty)
    
        

print(len(tall))
