import java.util.Scanner;

class Templates {
    public static void main(String[] args) {
    }

    public static void expressionStatement() {
        int x = 3, y = 2, z;
        // rel ops
        z = (x == y) ? 1 : 0;
        z = (x != y) ? 1 : 0;
        z = (x < y) ? 1 : 0;
        z = (x <= y) ? 1 : 0;
        z = (x > y) ? 1 : 0;
        z = (x >= y) ? 1 : 0;

        // mul ops
        z = x * y;
        z = x / y;
        z = x % y;
        z = ((x != 0) && (y != 0)) ? 1 : 0;

        // add ops
        z = x + y;
        z = x - y;
        z = ((x != 0) || (y != 0)) ? 1 : 0;

        // misc
        z = -x;
        z = !(x != 0) ? 1 : 0;
    }

    public static void ifElseStatement() {
        int i = 10;
        int a = 0;
        if (i > 0) {
            a = a + 3;
        } else {
            a = a - 3;
        }
    }

    public static int returnStatement() {
        int a = 0;
        if (a > 5) {
            return a;
        }

        a = 10;
        return a;
    }

    public static void whileStatement() {
        int a = 0;
        while (a < 5) {
            a += 1;
        }
    }

    public static void readStatement() {
        int i;
        Scanner scan = new Scanner(System.in);
        i = scan.nextInt();
    }

    public static void writeStatement() {
        String test = "test";
        System.out.print(test);
    }

    public static void newLineStatement() {
        System.out.println();
    }
}