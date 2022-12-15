with open("input", "r") as f:
    data = f.readlines()

data = [x.strip() for x in data]

grid = [[ 0 for x in range(len(data[0]))] for y in range(len(data))]

sx, sy = 0, 0
for x in range(len(data)):
    for y in range(len(data[x])):
        if data[x][y] == "S":
            pass
        elif data[x][y] == "E":
            sx, sy = x, y
            grid[x][y] = 25
        else:
            grid[x][y] = ord(data[x][y]) - ord("a")


visited = [[False for x in range(len(data[0]))] for y in range(len(data))]

qu = [(sx, sy, 0)]
visited[sx][sy] = True
dx, dy = [0, 0, 1, -1], [1, -1, 0, 0]
while qu:
    x, y, d = qu.pop(0)
    if grid[x][y] == 0:
        print(d)
        break
    for i in range(4):
        nx, ny = x + dx[i], y + dy[i]
        if 0 <= nx < len(data) and 0 <= ny < len(data[0]) and not visited[nx][ny]:
            if grid[nx][ny] >= grid[x][y] - 1:
                visited[nx][ny] = True
                qu.append((nx, ny, d + 1))