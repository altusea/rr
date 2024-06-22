use either::Either;

fn main() {
    let r: Either<&str, bool> = Either::Right(false);
    let l: Either<&str, i32> = Either::Left("err msg");
    println!("{}", r);
    println!("{}", l);
    let rs = serde_json::to_string(&r).unwrap();
    println!("{}", rs);
    let ls = serde_json::to_string(&l).unwrap();
    println!("{}", ls);
    let r1: Either<&str, bool> = serde_json::from_str(&rs).unwrap();
    let l1: Either<&str, i32> = serde_json::from_str(&ls).unwrap();
    println!("{:?}", r1.right());
    println!("{:?}", l1.left());
}
