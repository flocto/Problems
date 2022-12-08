with open("input", "r") as f:
    lines = f.readlines()

alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

total = 0
for line in lines:
    s1, s2 = line[:len(line)//2], line[len(line)//2:]
    s1 = set(s1)
    s2 = set(s2)
    
    both = list(s1.intersection(s2))[0]
    total += alphabet.index(both) + 1

print(total)
    