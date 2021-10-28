use std::collections::HashMap;
use std::process::Command;
use std::{thread, time};

use assert_cmd::prelude::*;
use reqwest::header::HeaderValue;
use reqwest::StatusCode;

use serde::Deserialize;

mod common;

#[derive(Deserialize)]
struct PostArticleResult {
    uuid: uuid::Uuid,
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
struct Article {
    pub uuid: uuid::Uuid,
    pub content: String,
}

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
    // waiting for the service to bootstrap
    thread::sleep(time::Duration::from_secs(2));
    // ping
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(format!("http://{}/api/v1/system/ping", bind))
        .send()
        .unwrap()
        .text()
        .unwrap();
    assert_eq!("pong", resp);
    // create articles
    let mut uuids = vec![];
    for i in 0..200 {
        let mut map = HashMap::new();
        map.insert("content", format!("# rust! [{}]", i));
        let resp = client
            .post(format!("http://{}/api/v1/article", bind))
            .json(&map)
            .send()
            .unwrap();
        let status = resp.status();
        assert_eq!(StatusCode::OK, status);
        let result = resp.json::<PostArticleResult>().unwrap();
        uuids.push(result.uuid);
    }
    assert_eq!(200, uuids.len());
    // list articles
    let resp = client
        .get(format!("http://{}/api/v1/article?start=0&end=19", bind))
        .send()
        .unwrap();
    let headers = resp.headers().clone();
    let header = headers.get("Content-Range").unwrap();
    let values = resp.json::<Vec<Article>>().unwrap();
    assert_eq!(20, values.len());
    assert_eq!(
        uuids.iter().take(20).copied().collect::<Vec<uuid::Uuid>>(),
        values.iter().map(|i| i.uuid).collect::<Vec<uuid::Uuid>>()
    );
    assert_eq!(
        vec![
            "# rust! [0]",
            "# rust! [1]",
            "# rust! [2]",
            "# rust! [3]",
            "# rust! [4]"
        ],
        values
            .iter()
            .take(5)
            .map(|i| i.content.as_str())
            .collect::<Vec<&str>>()
    );
    assert_eq!(&HeaderValue::from_str("article 0-19/200").unwrap(), header);
    let resp = client
        .get(format!("http://{}/api/v1/article?start=1&end=20", bind))
        .send()
        .unwrap();
    let headers = resp.headers().clone();
    let header = headers.get("Content-Range").unwrap();
    assert_eq!(&HeaderValue::from_str("article 1-20/200").unwrap(), header);
    let resp = client
        .get(format!("http://{}/api/v1/article?start=300&end=310", bind))
        .send()
        .unwrap();
    let headers = resp.headers().clone();
    let header = headers.get("Content-Range").unwrap();
    assert_eq!(
        &HeaderValue::from_str("article 300-310/200").unwrap(),
        header
    );
    let values = resp.json::<Vec<Article>>().unwrap();
    assert_eq!(0, values.len());
    // get by uuid
    let uuid = uuids.get(0).unwrap();
    let resp = client
        .get(format!("http://{}/api/v1/article/{}", bind, uuid))
        .send()
        .unwrap()
        .json::<Article>()
        .unwrap();
    assert_eq!(
        Article {
            uuid: *uuid,
            content: "# rust! [0]".to_string()
        },
        resp
    );
    let resp = client
        .get(format!(
            "http://{}/api/v1/article/{}",
            bind,
            uuid::Uuid::new_v4()
        ))
        .send()
        .unwrap()
        .status();
    assert_eq!(StatusCode::NOT_FOUND, resp);
    // delete by uuid
    let resp = client
        .delete(format!("http://{}/api/v1/article/{}", bind, uuid))
        .send()
        .unwrap();
    assert_eq!(StatusCode::OK, resp.status());
    let resp = client
        .get(format!("http://{}/api/v1/article/{}", bind, uuid))
        .send()
        .unwrap();
    assert_eq!(StatusCode::NOT_FOUND, resp.status());
}
