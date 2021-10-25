# inclusion-articles

## How to use it?

```rust
# use testcontainers::{clients, images, Docker};
use inclusion_articles::api::{
    Article, Create, DeleteByUuid, FetchByUuid, FetchPage, FetchTotal, Update,
};
use inclusion_articles::connection::ConnectionPool;

use std::str::FromStr;

# let docker = clients::Cli::default();
# let node = docker.run(
#     images::generic::GenericImage::new("library/postgres:13")
#         .with_wait_for(images::generic::WaitFor::message_on_stderr(
#             "database system is ready to accept connections",
#         ))
#         .with_env_var("POSTGRES_DB", "articlesdb")
#         .with_env_var("POSTGRES_USER", "rusty")
#         .with_env_var("POSTGRES_PASSWORD", "rusty"),
# );
# let db_url = format!(
#     "postgres://rusty:rusty@localhost:{}/articlesdb",
#     node.get_host_port(5432).unwrap()
# );
// connect to the api
let pool: ConnectionPool = db_url.parse().unwrap();
let connection = pool.connect().unwrap();

// create an article
let mut article = Article::default().create(&connection).unwrap();

// get an article by uuid
assert_eq!(
    Some(article.clone()),
    Article::fetch_by_uuid(&connection, &article.uuid).unwrap()
);

// update the article
article.content = "# Updated!".to_string();
let article = article.update(&connection).unwrap();
assert_eq!("# Updated!".to_string(), article.content);
assert_eq!(
    Some(article.clone()),
    Article::fetch_by_uuid(&connection, &article.uuid).unwrap()
);

// delete the article
Article::delete_by_uuid(&connection, &article.uuid).unwrap();
assert_eq!(
    None,
    Article::fetch_by_uuid(&connection, &article.uuid).unwrap()
);

// create several articles
let contents = (0..100)
    .into_iter()
    .map(|i| {
        let article = Article::default().content(format!("content {}", i));
        article.create(&connection).unwrap().content
    })
    .collect::<Vec<String>>();

// fetch page
assert_eq!(
    contents,
    Article::fetch_page(&connection, 0, 120)
        .unwrap()
        .into_iter()
        .map(|a| a.content)
        .collect::<Vec<String>>()
);

// fetch smaller page
assert_eq!(
    (0..80)
        .into_iter()
        .map(|i| { format!("content {}", i) })
        .collect::<Vec<String>>(),
    Article::fetch_page(&connection, 0, 79)
        .unwrap()
        .into_iter()
        .map(|a| a.content)
        .collect::<Vec<String>>()
);

// fetch total articles
assert_eq!(100, Article::fetch_total(&connection).unwrap());
```

## Database migrations

* Make sure you are at the same directory as this `README.md` file.
* You create a new migration with `diesel migration generate <migration_name_here>`.
* You run it on new DBs with `diesel migration run --database-url postgres://rusty:rusty@localhost:5432/articlesdb`. 
  The application does that automatically on `cargo run`.
* You rerun it on existing DBs with `diesel migration redo`
