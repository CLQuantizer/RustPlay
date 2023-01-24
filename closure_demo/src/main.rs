fn main() {
    let num: u8 = 127;
    let print_num = |name: &str, x:u8|{
        println!("{}: {}",name, x);
    };

    let print_res = |adder: u8|{
        print_num("adder", adder);
        print_num("num", num);
        println!("num+adder={}",adder+num);
    };
    print_res(8);
}
