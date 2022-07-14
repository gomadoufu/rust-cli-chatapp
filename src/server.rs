#![allow(dead_code)]
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop {
        //メッセージを待つ
        let mut line = String::new();
        if let Ok(n) = reader.read_line(&mut line) {
            if n > 0 {
                tx.send(line).unwrap();
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}

fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
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

fn server_loop(
    server: TcpListener,
    tx: Sender<String>,
    rx: Receiver<String>,
    mut clients: Vec<TcpStream>,
) {
    loop {
        //クライアントからのメッセージを待ち受ける
        if let Ok((client, addr)) = server.accept() {
            println!("クライアントが接続しました: {}", addr);
            clients.push(client.try_clone().unwrap());
            start_thread(client, tx.clone());
        }

        //スレッド間通信のメッセージを待ち受ける
        if let Ok(msg) = rx.try_recv() {
            println!("全員に送信しました: {}", msg.trim());
            //全クライアントに送信(同期を取る)
            clients = send_all(clients, &msg);
        }
        thread::sleep(Duration::from_millis(100));
    }
}

pub fn start_server(server_addr: &str) {
    //スレッド間通信を用意
    let (tx, rx) = mpsc::channel::<String>();

    //サーバーを起動
    let server = TcpListener::bind(server_addr).expect("サーバーの起動に失敗");
    server.set_nonblocking(true).expect("利用不可");
    println!("サーバー起動中: {}", server_addr);
    //クライアント一覧のベクタ
    let clients: Vec<TcpStream> = Vec::new();
    //メインループ
    server_loop(server, tx, rx, clients);
}
