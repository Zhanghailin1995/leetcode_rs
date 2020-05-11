struct Cat {
    pub c: i32
}

struct Bar {
    pub b: i32
}

struct Foo<'a> {
    bar: &'a Bar,
    cat: &'a Cat
}

fn main() {
    let bar: &Bar = &Bar{ b: 123 };
    let cat: &Cat = &Cat{ c: 234 };
    let mut foo = Foo {bar,cat};
    {
        //let local_cat = Cat{c: 666};
        let local_cat = &Cat{c: 666};
        foo.cat = &local_cat;
        println!("{}", foo.cat.c)
    }
    let cat = foo.cat;
    println!("{}", cat.c);
}
