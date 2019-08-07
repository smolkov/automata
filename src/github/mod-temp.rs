pub mod input;
pub mod output;

pub mod posts;
pub mod fetcher;
pub mod backup;

pub mod query;
pub mod page_gen;
mod issue_or_pr;
mod issues_with_label;
mod latest_tag;
// use tera;
// use tera::{Context, Tera};
// use chrono::Utc;
// use std::fs;
// use posts::Post;
// use input::Item;


use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use graphql_client::Response;
use reqwest::Client;
use std::error::Error;
use std::fmt::{self, Display};
use std::path::Path;
use std::collections::HashMap;
mod filters;

const DATA_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data.yml");
const POSTS_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/posts.yml");
const CACHE_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/cache.json");

const INDEX_FILE: &str = "index.html";

pub type IssueId = u32;
impl Error for QueryError {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Issue {
    pub number: u32,
    pub title: String,
    pub open: bool,
}


lazy_static! {
    static ref OUT_DIR: &'static Path = Path::new("out");
    static ref RFC_REPO: Repo = Repo::new("rust-lang", "rfcs");
    static ref RUSTC_REPO: Repo = Repo::new("rust-lang", "rust");
}

#[derive(Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Repo {
    pub owner: String,
    pub name: String,
}

impl Repo {
    pub fn new(owner: &str, name: &str) -> Self {
        Self {
            owner: owner.to_string(),
            name: name.to_string(),
        }
    }
}

impl Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

pub struct GitHubQuery<'a> {
    client: &'a Client,
    token: &'a str,
}

impl<'a> GitHubQuery<'a> {
    pub fn new(client: &'a Client, token: &'a str) -> Self {
        GitHubQuery { client, token }
    }

    fn send_query<Q, D>(&self, name: &'static str, query: &Q) -> Result<D, Box<dyn Error>>
    where
        Q: Serialize,
        for<'de> Response<D>: Deserialize<'de>,
    {
        let resp = self
            .client
            .post("https://api.github.com/graphql")
            .bearer_auth(self.token)
            .json(query)
            .send()?
            .json::<Response<D>>()?;
        if let Some(errors) = resp.errors {
            Err(QueryError { name, errors })?;
        }
        Ok(resp.data.unwrap())
    }
}

#[derive(Debug)]
struct QueryError {
    name: &'static str,
    errors: Vec<graphql_client::Error>,
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "query '{}' fails:", self.name)?;
        for error in self.errors.iter() {
            writeln!(f, "{}", error)?;
        }
        Ok(())
    }
}






// pub fn generate(items: &HashMap<String, Vec<Item>>, posts: &[Post]) -> Result<(), Box<dyn Error>> {
//     let mut tera = Tera::new("templates/**/*.html")?;
//     tera.register_filter("codify", filters::codify);
//     tera.register_filter("pr_url", filters::pr_url);
//     tera.register_filter("issue_url", filters::issue_url);
//     let mut context = Context::new();
//     context.insert("items", &items);
//     context.insert("posts", &posts);
//     context.insert("time", &Utc::now().to_rfc2822());
//     let html = tera.render(INDEX_FILE, &context)?;
//     fs::write(OUT_DIR.join(INDEX_FILE), html)?;
//     Ok(())
// }
