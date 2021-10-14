use std::*;

fn read_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    input
}

fn write_stdout(message: &str) -> () {
    print!("あなたは、"); //改行なし
    println!("{}と入力しました。", message); //改行あり

    ()
}

fn main() -> io::Result<()> {
    println!("please input keys");

    let input = read_stdin();
    let message = input.trim();
    write_stdout(message);
    Ok(())
}
