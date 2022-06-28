fn main(){
    let s = format!("{}{}",
            "There is more happniness in giving",
            "than there is in recwiving");
    let res = s.find(|c:char| c.to_ascii_uppercase() == 'S');
    match res{
        Some(i) => println!("S={}B", i),
        None => println!("None"),
    };
}