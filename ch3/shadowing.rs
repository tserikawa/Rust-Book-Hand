fn main(){
    // 使わない場合
    {
        let mut v = 300;
        v = v + 5;
        println!("{}", v);
    }

    // 使う場合
    {
        let v = 300;
        let v = v + 5;
        println!("{}", v);
    }
}