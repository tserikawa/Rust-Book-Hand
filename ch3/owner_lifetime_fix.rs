fn gen_message() -> String{
    let msg = String::from("過ちを見過ごす人は美しい");
    return msg;  // 実体を返す
}

fn main(){
    let m = gen_message();
    println!("{}", m);
}