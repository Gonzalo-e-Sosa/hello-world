fn hello_name_string(name: String) {
    println!("Hello {name}!");
    println!("From: hello_name_string\n");
}

fn hello_name_str(name: &std::primitive::str) {
    println!("Hello {name}!");
    println!("From: hello_name_str\n");
}

fn hello_name_v_char(a: Vec<std::primitive::char>) {
    print!("Hello ");
    for i in a {
        print!("{}", i);
    }
    println!();

    println!("From: hello_name_v_char\n");
}

fn main() {
    println!("Hello, world!\n");

    let name: &str = "Gonza";

    hello_name_string(name.to_string());
    hello_name_str(name);

    let mut v: Vec<std::primitive::char> = [].to_vec();

    let mut it: std::str::Chars = name.chars();
    for _ in 0..name.len() {
        v.push(it.next().unwrap());
    }

    hello_name_v_char(v);
}
