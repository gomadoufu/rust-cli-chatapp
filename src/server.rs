use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn start_server(server_addr: &str) {
    //スレッド間通信を用意
    let (tx, rx) = mpsc::channel::<String>();
    //クライアント一覧のベクタ
    //let mut clients: Vec<TcpStream> = Vec::new();

    //サーバーを起動
    let server = TcpListener::bind(server_addr).expect("サーバーの起動に失敗");
    server.set_nonblocking(true).expect("利用不可");
    println!("サーバー起動中: {}", server_addr);
}

fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop {
        //メッセージを待つ
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(n) if n > 0 => {
                //クライアントからのメッセージを受け取る
                tx.send(line).unwrap();
            }
            Err(_) => {
                //クライアントが切断した
                break;
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}

fn send_all(clients: Vec<TcpStream>, msg: &str) -> Vec<TcpStream> {
    let mut collector = vec![];
    for mut socket in clients.into_iter() {
        //文字列をバイト列に変換して送信
        let bytes = String::from(s).into_bytes();
        if let Err(e) = socket.write_all(&bytes) {
            println!("送信エラー: {}", e);
            continue;
        }
        collector.push(socket);
    }
    collector //所有権を戻す
}
