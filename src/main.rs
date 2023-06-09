use autorust::Value;

pub fn main() {
    let a = Value::from(-4.0).with_label("a");
    let b = Value::from(2.0).with_label("b");
    let c = (a.clone() + b.clone()).with_label("c");
    let d = (a.clone() * b.clone()).with_label("d");
    // c += 1 + c + (-a);
    // d += d * 2 + (b + a).relu();
    // d += 3 * d + (b - a).relu();
    let e = c.clone() - d.clone();
    let f = (e.pow(&Value::from(2.0))).with_label("f");
    // let g = (&f / 2.0).with_label("g");
    // g += 10.0 / f;

    println!("a.data: {}", a.data());
    println!("b.data: {}", b.data());
    println!("c.data: {}", c.data());
    println!("d.data: {}", d.data());
    println!("e.data: {}", e.data());
    println!("f.data: {}", f.data());
    // println!("g.data: {}", g.data());

    // g.backward();


    println!("Hello World");

    let x1 = Value::from(2.0).with_label("x1");
    let x2 = Value::from(0.0).with_label("x2");

    let w1 = Value::from(-3.0).with_label("w1");
    let w2 = Value::from(1.0).with_label("w2");

    let b = Value::from(6.8813735870195432).with_label("b");

    let x1w1 = (x1 * w1).with_label("x1w1");
    let x2w2 = (x2 * w2).with_label("x2w2");
    println!("{}", x1w1.data());
    println!("{}", x2w2.data());


    let x1w1x2w2 = (x1w1 + x2w2).with_label("x1w1x2w2");
    println!("{}", x1w1x2w2.data());

    let n = (x1w1x2w2 + b).with_label("n");
    let o = n.tanh().with_label("o");

    o.backward();
}