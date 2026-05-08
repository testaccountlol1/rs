fn main() {
    let vec: Vec<i32> = [1,2,3,5,8,4,0,10,23,30].to_vec();
    println!("{:#?}",math::sr::bins(vec.clone(),30));

    println!("{:#?}",math::sr::srt(vec));
}
