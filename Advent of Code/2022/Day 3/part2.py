with open("input", "r") as f:
    lines = f.readlines()

alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

total = 0
for i in range(0, len(lines), 3):
    s1, s2, s3 = lines[i].strip(), lines[i+1].strip(), lines[i+2].strip()
    s1, s2, s3 = set(s1), set(s2), set(s3)
    inter = s1.intersection(s2, s3)
    total += alphabet.index(list(inter)[0]) + 1
print(total)
    