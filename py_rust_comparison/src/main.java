import java.util.*;

class Dog {
    int id;

    Dog(int id) {
        this.id = id;
    }

    public static void main(String[] args) {
        Stack<Dog> l = new Stack<>();
        l.add(new Dog(1));
        l.add(new Dog(2));
        l.add(new Dog(3));
        Dog dog3 = l.peek();
        l.pop();
        System.out.println(dog3.id);
    }
}