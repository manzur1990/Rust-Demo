use std::env;

fn main(){
    let args: Vec<String> = env::args().collect()

    for i in &args {
        println!("{}", i);
    }

    println!("{:?}", args);
}

ip_sniffer.exe -h
ip_sniffer.exe -j 100 192.168.1.1
ip_sniffer.exe 192.168.1.1