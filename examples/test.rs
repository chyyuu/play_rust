fn main() {
    // 获取所有权
    let msg1 = String::from("Hello String!");
    let say_hello = || {
        let msg = msg1;
        println!("{}", msg);
    };
    say_hello(); // Hello String!
                 //println!("{}", msg1); // 此时无法访问`msg1`，因为它的所有权已经转移到闭包了

    // 不可变借用
    let msg2 = String::from("Hello &String!");
    let say_hello = || {
        let msg = &msg2;
        println!("{}", msg);
    };
    say_hello(); // Hello &String!
    println!("{}", msg2); // Hello &String!

    // 可变借用
    let mut msg3 = String::from("Hello &mut String");
    let mut say_hello = || {
        let msg = &mut msg3;
        msg.push_str("!");
        println!("{}", msg);
    };

    say_hello(); // Hello &mut String!
    say_hello(); // Hello &mut String!!
                 //msg3.push_str("--");
    let mut say_hello2 = || {
        let msg = &mut msg3;
        msg.push_str("--");
        println!("{}", msg);
    };
    say_hello2();
    println!("{}", msg3); // Hello &mut String!!
}
