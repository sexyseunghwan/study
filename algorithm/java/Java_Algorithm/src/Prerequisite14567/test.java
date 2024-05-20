package Prerequisite14567;

import java.io.*;
import java.util.*;

public class test {
    public static void main(String[] args) throws Exception {
        
        Map<Integer, List<Integer>> adjList = new HashMap<>();
        
        for (int i = 1; i <= 10; i++) {
            adjList.put(i, new ArrayList<Integer>());
        }
        
        adjList.get(1).add(1);
        
    }
}
