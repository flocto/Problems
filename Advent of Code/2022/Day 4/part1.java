import java.util.*;
import java.io.*;

public class part1 {
    public static void main(String[] args) throws IOException {
        Scanner in = new Scanner(new File("input"));
        String line;
        int c = 0;
        while (in.hasNextLine() && ((line = in.nextLine()) != null)) {
            String[] parts = line.split(",");
            int[] p1 = Arrays.stream(parts[0].split("-")).mapToInt(Integer::parseInt).toArray();
            int[] p2 = Arrays.stream(parts[1].split("-")).mapToInt(Integer::parseInt).toArray();

            if (p1[0] <= p2[0] && p1[1] >= p2[1]) {
                c++;
            }
            else if (p2[0] <= p1[0] && p2[1] >= p1[1]) {
                c++;
            }
        }
        System.out.println(c);
    }
}