extern crate slack_hook;
use slack_hook::{Slack, PayloadBuilder};
use std::time::Duration;
use std::thread;

#[no_mangle]
pub extern fn send_msg_ffi() {
    thread::spawn(move || {
        send_msg();
    });
}

fn send_msg() {
    println!("rust-slack-start");
    let webhook_url = "<自分のslackのwebhook url>";

    let slack = Slack::new(webhook_url).unwrap();
    let p = PayloadBuilder::new()
        .text("rustrustrust")
        .channel("<#ではじまるチャンネル名 or @ではじまる宛先ユーザー名>")
        .username("rust-slack-notify")
        .build()
        .unwrap();

    let res = slack.send(&p);
    match res {
        Ok(()) => println!("ok"),
        Err(e) => println!("ERR: {:?}", e)
    }

    // あえて3秒まってみる
    thread::sleep(Duration::from_millis(3000));
    println!("rust-slack-end");
}
