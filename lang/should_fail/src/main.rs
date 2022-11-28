fn main() {
    let r;

    {
        let x = 5;
        r = &x; // error[E0597]: `x` does not live long enough
    }

    println!("r: {}", r);
}
