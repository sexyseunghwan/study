using System;


public class Window {
    public double size;
    public Window() {
        size = 3.14;
        Console.WriteLine("\t{0} is created.", this);
    }
    ~Window() {
        size = 0.0;
        Console.WriteLine("\t{0} is destructed.", this);
    }
}

public class Apt {
    public Window Window { get; set; }
    public Apt() {
        Console.WriteLine("\t{0} is created.", this);
    }
    ~Apt() {
        Console.WriteLine("\t{0} is destructed.", this);
    }
}

public class Program {
    public static void Main() {
        Console.WriteLine("Program Start");
        var window = new Window();
        var apt = new Apt();
        apt.Window = window;
        window = null;
        apt = null;
        GC.Collect();
        GC.WaitForFullGCComplete();
        Console.WriteLine("Program End");    
    }
}
