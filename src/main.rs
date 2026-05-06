fn main() {
    let a = math::op::revi(&mut 92);
    std::println!("{}",a);
    let b = math::pow(&5,&5);
    std::println!("{}",b);
    let c = math::neq(&1);
    std::println!("{}",c);
    let d = math::ab(&c);
    std::println!("{}",d);
}