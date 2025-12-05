import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;

public class App {
    public static void main(String[] args) throws Exception {
        String rawString = Files.readString(Path.of("day4/App/input.txt"), StandardCharsets.UTF_8);
        Warehouse w = new Warehouse(rawString);
        
        // System.out.println(w.countFindablePaper());
        System.out.println(w.findAndRemovePaper());
    }
}
