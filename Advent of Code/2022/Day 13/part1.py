with open("input", "r") as f:
    data = f.readlines()

data = [x.strip() for x in data]

def handle(left, right):
    if type(left) == int and type(right) == int:
        return left - right

    if type(left) == int and type(right) == list:
        left = [left]
    if type(left) == list and type(right) == int:
        right = [right]
    
    
    # both lists
    i = 0 
    while i < len(left) and i < len(right):
        rec = handle(left[i], right[i])
        # print(left, right, rec, sep="\n")

        if type(rec) == bool:
            return rec
        if rec != 0:
            return rec < 0
        i += 1
    # print(left, right, len(left) - len(right))
    return len(left) - len(right)



total = 0
for i in range(0, len(data), 3):
    l1 = eval(data[i])
    l2 = eval(data[i+1])

    ordered = handle(l1, l2)
    if type(ordered) == int:
        ordered = ordered < 0
    # print(l1, "\n", l2, ordered)
    if ordered:
        total += (i // 3) + 1
        # print((i // 3) + 1)
    # print("\n\n")
    

print(total)