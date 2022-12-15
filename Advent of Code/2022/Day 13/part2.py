with open("input", "r") as f:
    data = f.readlines()

data = [x.strip() for x in data]

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

def wrap(left, right):
    # compare function
    ordered = handle(left, right)
    if type(ordered) == int:
        return ordered 
    return -1 if ordered else 1


total = 0
packets = [[[2]], [[6]]]
for i in range(0, len(data), 3):
    l1 = eval(data[i])
    l2 = eval(data[i+1])
    packets.append(l1)
    packets.append(l2)

import functools
packets.sort(key=functools.cmp_to_key(wrap))

d1 = [x for x in range(len(packets)) if packets[x] == [[2]]][0] + 1
d2 = [x for x in range(len(packets)) if packets[x] == [[6]]][0] + 1
print(d1 * d2)