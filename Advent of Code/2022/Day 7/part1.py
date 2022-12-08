# actually implementation hell What the f**k

with open("input", "r") as f:
    data = f.readlines()

data = [x.strip() for x in data]
# first line always cd /, shows up nowhere else
data = data[1:]

filesystem = {"/": {}}
pwd = "/"


i = 0
while i < len(data):
    line = data[i]
    if line.startswith("$"): # this should always be true, ls reading is done below
        cmd = line[2:].split(" ")
        if cmd[0] == "cd":
            if cmd[1] == "..":
                if pwd == "/":
                    continue
                pwd = pwd[:pwd.rfind("/")]
            else:
                if pwd == "/":
                    pwd = pwd + cmd[1]
                else:
                    pwd += "/" + cmd[1]

        elif cmd[0] == "ls":
            ls = []
            i += 1
            while not (line := data[i]).startswith("$"):
                ls.append(line)
                i += 1 
                if i == len(data):
                    break
            i -= 1

            for file in ls:
                file = file.split(" ")
                if file[0] == "dir":
                    filesystem[pwd][file[1]] = (0, True)
                else:
                    filesystem[pwd][file[1]] = (int(file[0]), False)
                
    if pwd not in filesystem:
        filesystem[pwd] = {}

    i += 1

def fix(dir):
    temp = 0
    for file in filesystem[dir]:
        if filesystem[dir][file][1]:
            if dir == "/":
                rec = fix(dir + file)
            else:
                rec = fix(dir + "/" + file)
            filesystem[dir][file] = (rec, True)
            temp += rec
        else:
            temp += filesystem[dir][file][0]
    
    return temp

print(fix("/"))

total = 0
for dir in filesystem:
    for file in filesystem[dir]:
        if filesystem[dir][file][1] and filesystem[dir][file][0] <= 100000:
            total += filesystem[dir][file][0]

print(total)