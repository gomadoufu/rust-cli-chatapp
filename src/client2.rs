use std::io::{stdin, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    //サーバーのアドレスを指定
    let server_addr = "localhost:8080";
    //サーバーと接続
    let mut socket = TcpStream::connect(server_addr).expect("サーバーと接続できません");
    socket.set_nonblocking(true).expect("利用不可");
    println!("サーバーと接続しました");

    //受診用のスレッドを開始
    start_thread(socket.try_clone().unwrap());
    //標準入力からユーザー名を得る
    let user = input("ユーザー名を入力してください: ");
    println!("{}さん、こんにちは！", user);
    println!("{}さん、メッセージを入力してください", user);
    loop {
        //標準入力から入力を得てサーバーに送信
        let msg = input("> ");
        let msg = format!("{}> {}\n", user, msg);
        let buf = msg.as_bytes();
        socket.write_all(buf).unwrap();
    }
}

fn start_thread(socket: TcpStream) {
    let mut reader = BufReader::new(socket);
    thread::spawn(move || loop {
        //メッセージを待つ
        let mut line = String::new();
        if let Ok(n) = reader.read_line(&mut line) {
            if n > 0 {
                println!("{}", line);
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}

//標準入力から文字列を得る
fn input(msg: &str) -> String {
    if msg != "" {
        println!("{}", msg);
    }
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("入力できません");
    String::from(buf.trim())
}
