using System;
using System.Collections.Generic;
using System.Linq;

class HelloWorld {
    static void Main() {
        
        string text1 = "마켓";
        string text2 = "마켓 컬리 잠실";

        var vector1 = GetVector(text1);
        var vector2 = GetVector(text2);
        

        Console.WriteLine("vector1: {0}", vector1);
        Console.WriteLine("vector2: {0}", vector2);
        double cosineSimilarity = CosineSimilarity(vector1, vector2);
        Console.WriteLine("코사인 유사도: {0}", cosineSimilarity);
    }

    static Dictionary<string, int> GetVector(string text)
    {
        var words = text.Split(' ');
        var vector = new Dictionary<string, int>();
        
        foreach (var word in words)
        {
            if (vector.ContainsKey(word))
                vector[word]++;
            else
                vector[word] = 1;
        }

        return vector;
    }

    static double CosineSimilarity(Dictionary<string, int> vector1, Dictionary<string, int> vector2)
    {
        var intersection = vector1.Keys.Intersect(vector2.Keys);
        double dotProduct = intersection.Sum(k => vector1[k] * vector2[k]);

        double magnitude1 = Math.Sqrt(vector1.Values.Sum(v => v * v));
        double magnitude2 = Math.Sqrt(vector2.Values.Sum(v => v * v));

        if (magnitude1 == 0 || magnitude2 == 0)
            return 0;

        return dotProduct / (magnitude1 * magnitude2);
    }
}
