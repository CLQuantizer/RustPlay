class Dog {
    id: number;
    constructor(id: number) { this.id = id; }
}

let l: Dog[] = [new Dog(1), new Dog(2), new Dog(3)];
let dog3: Dog = l[l.length-1];
l.pop();
console.log(dog3.id);