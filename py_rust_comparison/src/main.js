var Dog = /** @class */ (function () {
    function Dog(id) {
        this.id = id;
    }
    return Dog;
}());
var l = [new Dog(1), new Dog(2), new Dog(3)];
var dog3 = l[l.length - 1];
l.pop();
console.log(dog3.id);
