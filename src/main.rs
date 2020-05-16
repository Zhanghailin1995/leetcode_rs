/*struct Cat {
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
}*/

use std::fmt::Debug;
use std::marker::PhantomData;
use std::ptr::replace;

static mut G2 : i32 = 4;

fn main() {
    /*let u = {
        let t = Test {
            v: Box::new(1)
        };
        //User::<Test>::create_user(&t)
        User {
            age: 1,
            data: &t as *const Test,
            _marker: PhantomData,
        }
    };
    println!("data: {:?}", unsafe { &(*u.data).v });*/
    let local_var;
    unsafe {
        local_var = G2;
        println!("{}", local_var);
    }
    println!("{}", local_var);
}

#[derive(Debug)]
struct Test {
    v: Box<i32>
}

#[derive(Debug)]
struct User<'a, T: Debug + 'a> {
    age: u8,
    data: *const T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: Debug> User<'a, T> {
    fn create_user(t: &T) -> User<T> {
        User {
            age: 1,
            data: t as *const T,
            _marker: PhantomData,
        }
    }
}


