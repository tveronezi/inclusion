mod common;

use assert_cmd::prelude::*;
use std::process::Command;
use std::{thread, time};

#[test]
fn test_crud() {
    let client = common::Client::default();
    let stack = client.new_stack();
    let db_url = stack.articlesdb_url();
    let bind = format!(
        "127.0.0.1:{}",
        portpicker::pick_unused_port().expect("No ports free")
    );
    let bind_thread = bind.clone();
    thread::spawn(move || {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.env("articles_db_url", db_url);
        cmd.env("bind", bind_thread);
        let _ = cmd.unwrap();
    });
    thread::sleep(time::Duration::from_secs(2));
    let resp = reqwest::blocking::get(format!("http://{}/api/v1/system/ping", bind))
        .unwrap()
        .text()
        .unwrap();
    assert_eq!("pong", resp);
}
