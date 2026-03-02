let const = 5;

type Dog {
    fn init(a) {
        self.a = a;
        self.b = const;
    }
}

let dog = Dog(3);
let dog2 = dog;
println(dog.a);
println(dog2.a);
println(dog.b);
println(dog2.b);
