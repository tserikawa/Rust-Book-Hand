fn main() {
    for y in 1..10 {
        for x in 1..10 {
            // ":3"は右寄せ
            print!("{:3},", x * y);
        }
        println!("");
    }
}
