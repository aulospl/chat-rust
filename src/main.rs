use std::env;
use std::process;

fn main() {
    let inc_msg = String::from("Incorrect usage. Please refer to the help page (-h) for instructions.");
    
    let help_msg = String::from("Usage: chat-rust <server> <ip/port> [<username>]\n\n\t -h\t show this message.\n\t server\t 'server' if you wish to start a server, 'client' if you wish to join as a client.\n\t ip/port\t The ip and port you wish to connect to or open.\n\t username\t CLIENT ONLY. Choose a unique username to log-in as.");

    let args: Vec<String> = env::args().collect();

    match args.len(){
        1 => {
            println!("{}", &inc_msg);
            process::exit(1);
        },
        _ => {
            if args.len() > 4{
                println!("{}", &inc_msg);
                process::exit(1);
            }
            for i in &args{
                if i.eq("-h"){
                    println!("{}", &help_msg);
                    process::exit(1);
                }
            }
        },
    }

}
