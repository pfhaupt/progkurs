// Compile: javac Main.java
// Run:     java Main
public class Main {
    private static class Test {
        int a = 10;
    }
    private static void someFunction(Test t) {
        System.out.println(t.a);
    }
    public static void main(String[] args) {
        Test t1 = new Test();
        Test t2 = t1;
        Test t3 = t1;
        t2.a = 5;
        someFunction(t1);
        someFunction(t3);
    }
}
