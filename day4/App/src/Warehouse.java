import java.util.List;
import java.util.ArrayList;

public class Warehouse {
    List<List<String>> grid;
    int[][] directions = {
    {-1, -1}, // up-left
    {-1,  0}, // up
    {-1,  1}, // up-right
    { 0, -1}, // left
    { 0,  1}, // right
    { 1, -1}, // down-left
    { 1,  0}, // down
    { 1,  1}  // down-right
};

    public Warehouse(String rawString) {
        List<List<String>> grid = new ArrayList<>();

        String[] lines = rawString.split("\n");
        for (int i = 0; i < lines.length; i++) {
            String line = lines[i];
            List<String> gridLine = new ArrayList<>();

            for (int j = 0; j < line.length(); j++) {
                gridLine.add(Character.toString(line.charAt(j)));
            }
            grid.add(gridLine);
        }
        this.grid = grid;
    }

    public void printGrid(){
        System.out.println(this.grid);
    }

    // part 1
    public int countFindablePaper() {
        int count = 0;
        for (int x = 0; x < this.grid.size(); x++) {
            List<String> row = this.grid.get(x);
            for (int y = 0; y < row.size(); y++) {
                if (!this.grid.get(x).get(y).equals("@")) {
                    // Not a paper
                    continue;
                }
                if (this.getSurroundingPaper(x, y) < 4) {
                    System.out.println(String.format("Findable (%d,%d)", x, y));
                    count++;
                }
            }
        }

        return count;
    }

    private int countAndRemoveFindablePaper() {
        int count = 0;
        for (int x = 0; x < this.grid.size(); x++) {
            List<String> row = this.grid.get(x);
            for (int y = 0; y < row.size(); y++) {
                if (!this.grid.get(x).get(y).equals("@")) {
                    // Not a paper
                    continue;
                }
                if (this.getSurroundingPaper(x, y) < 4) {
                    this.grid.get(x).set(y, "x");
                    count++;
                }
            }
        }

        return count;
    }

    // TODO: for an inefficient solution, could call part1 but have it return all the coords. 
    // remove those coords, then call part 1 again. 
    public int findAndRemovePaper() {
        int totalCount = 0;
        while (true) {
            int found = countAndRemoveFindablePaper();
            if (found == 0) {
                return totalCount;
            }
            totalCount += found;
        }
    }

    private int getSurroundingPaper(int x, int y) {
        int count = 0;
        for (int[] transformation : this.directions) {
            // Should length check, but try catch is more fun because I never get to use it ;)
            try {
                boolean hasAdjacent = this.hasPaper(x + transformation[0], y + transformation[1]);
                if (hasAdjacent) {
                    count++;
                }
            } catch (IndexOutOfBoundsException e) {}
        }
        return count;
    }

    private boolean hasPaper(int x, int y) {
        return this.grid.get(x).get(y).equals("@");
    }

}