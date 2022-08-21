use rand::Rng;
fn main(){
    let tmp = rand_int(0, 16);

    println!("Integer: {}", tmp);

    for i in 0..100{
        let a = rand_str();
        println!("{}",a)
    }

}

fn rand_int(min:i8,max:i8) -> i8{
    rand::thread_rng().gen_range(min..max)
}

fn rand_str() -> char{
    rand::thread_rng().gen_range(97_u8..123_u8) as char
}