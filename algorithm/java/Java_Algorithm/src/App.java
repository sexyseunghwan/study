import java.io.*;
import java.util.*;

public class App {
    static int[] map;
    public static void main(String[] args) throws Exception {
        
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));

        char[] chars = br.readLine().toCharArray();

        int[] arr = new int[chars.length];

        // for (int i = 0; i < arr.length; i++) {
        //     arr[i] = chars[i] - "0";
        // }

        for (int i = 0; i < arr.length; i++) {
            System.out.println(arr[i]);
        }

    }
}
