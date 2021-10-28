use std::rc::Rc;
use testcontainers::clients::Cli;
use testcontainers::images::generic::GenericImage;
use testcontainers::{clients, images, Container, Docker};

pub struct Client {
    docker: Rc<clients::Cli>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            docker: Rc::new(clients::Cli::default()),
        }
    }
}

pub struct Stack<'a> {
    articlesdb_node: Container<'a, Cli, GenericImage>,
}

impl Client {
    pub fn new_stack(&self) -> Stack {
        Stack {
            articlesdb_node: self.docker.run(
                images::generic::GenericImage::new("library/postgres:13")
                    .with_wait_for(images::generic::WaitFor::message_on_stderr(
                        "database system is ready to accept connections",
                    ))
                    .with_env_var("POSTGRES_DB", "articlesdb")
                    .with_env_var("POSTGRES_USER", "rusty")
                    .with_env_var("POSTGRES_PASSWORD", "rusty"),
            ),
        }
    }
}

impl Stack<'_> {
    pub fn articlesdb_url(&self) -> String {
        let port = self.articlesdb_node.get_host_port(5432).unwrap();
        format!("postgres://rusty:rusty@localhost:{}/articlesdb", port)
    }
}
