using System;

class HelloWorld {
    static void Main() {
        
        levenshteinDistance("마켓", "안녕");
    }

    static void levenshteinDistance(string s1, string s2) {
        
        int s1_len = s1.Length;
        int s2_len = s2.Length;

        int[ , ] matrix = new int[s1_len + 1, s2_len + 1];

        for (int i = 0; i <= s1_len; i++) 
        {
            matrix[0,i] = i;
            matrix[i,0] = i; 
        } 
        
        for (int i = 1; i < s1_len; i++)
        {
            for (int j = 1; j < s2_len; j++)
            {
                int cot = s1[i] == s2[j] ? 0 : 1;
                matrixs[i,j] = Math.Min(Math.Min(matrix[i - 1, j] + 1, matrix[i, j - 1] + 1), matrix[i - 1, j - 1] + cost);
            }
        }
        
        for (int i = 0; i < s1_len; i++)
        {
            for (int j = 0; j < s2_len; j++)
            {
                Console.Write(matrix[i,j] + " ");    
            }

            Console.WriteLine();
        }
    }   
}
