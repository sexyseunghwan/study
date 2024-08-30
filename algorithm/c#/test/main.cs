using System;

public class Parent
{
    protected string parentVariable = "This is a variable from the Parent.";
    protected int parent_int;

    public Parent() {
        Console.WriteLine("HAHA");
    }

    public void ShowVariable()
    {
        Console.WriteLine(parentVariable);
    }
}

public class Child : Parent
{
    public Child() : base() {}
    
    public void ShowParentVariable()
    {
        // 자식 클래스에서 부모 클래스의 변수 사용
        //Console.WriteLine(parentVariable);
        parent_int = 100;

        Console.WriteLine(parent_int);
    }
}

class Program
{
    static void Main()
    {
        Child child = new Child();
        child.ShowParentVariable();  // 부모 클래스의 변수를 출력
        child.ShowVariable();  // 부모 클래스의 메소드를 통해 변수를 출력
    }
}
