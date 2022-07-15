class Solution {
    static int[][] dxy = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};
    public int maxAreaOfIsland(int[][] grid) {
        boolean visited[][] = new boolean[grid.length][grid[0].length];
        int max = 0;
        for (int i = 0; i < grid.length; i++) {
            for (int j = 0; j < grid[0].length; j++) {
                if (grid[i][j] == 1 && !visited[i][j]) {
                    max = Math.max(max, dfs(grid, i, j, visited));
                }
            }
        }
        return max;
    }
    public int dfs(int[][] grid, int i, int j, boolean[][] visited) {
        int count = 0;
        visited[i][j] = true;
        for (int[] d : dxy) {
            int x = i + d[0];
            int y = j + d[1];
            if (x >= 0 && x < grid.length && y >= 0 && y < grid[0].length && grid[x][y] == 1 && !visited[x][y]) {
                count += dfs(grid, x, y, visited);
            }
        }
        return count + 1;
    }
}