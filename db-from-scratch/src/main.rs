use std::io::{self, BufRead, Write};

fn print_prompt() {
    print!("db > ");
    let _ = io::stdout().flush();
}

fn main() {
    let stdin = io::stdin();
    loop {
        print_prompt();
        let line = stdin.lock().lines().next().unwrap().unwrap();

        match line.as_str() {
            ".exit" => std::process::exit(0),
            _ => println!("Unrecognized command: '{}'", line),
        }
    }
}

// std::string read_input()
// {
//     std::string line;
//     std::getline(std::cin, line);
//     return line;
// }
//
// void print_prompt()
// {
//     std::cout << ("db > ");
// }
//
// int main(int, char*[])
// {
//     while (true) {
//         print_prompt();
//         const auto line = read_input();
//
//         if (line == ".exit") {
//             exit(EXIT_SUCCESS);
//         }
//         else {
//             std::cout << "Unrecoginized command: '" << line << "'.\n";
//         }
//     }
// }
