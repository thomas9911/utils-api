use std::io::Write;

use anyhow::Result;

#[derive(Default)]
struct Client {
    client: reqwest::Client,
    base_url: String,
}

#[derive(Clone, Debug)]
pub enum Output {
    U32,
    U64,
    U128,
    F32,
    F64,
}

impl std::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::U32 => write!(f, "u32"),
            Self::U64 => write!(f, "u64"),
            Self::U128 => write!(f, "u128"),
            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),
        }
    }
}

impl Default for Output {
    fn default() -> Output {
        Self::U32
    }
}

impl Client {
    pub fn new(base_url: &str) -> Client {
        let mut client = Self::default();
        client.base_url = base_url.to_string();

        client
    }

    async fn simple_request(
        &self,
        method: reqwest::Method,
        path: &str,
        text: &str,
    ) -> Result<String> {
        let out = self
            .client
            .request(method, format!("{}{path}", self.base_url))
            .body(text.to_string())
            .send()
            .await?;
        out.text().await.map_err(|e| e.into())
    }

    pub async fn minify_js(&self, javascript: &str) -> Result<String> {
        self.simple_request(
            reqwest::Method::POST,
            "/api/javascript/minifier",
            javascript,
        )
        .await
    }

    pub async fn random(&self, output: Option<Output>) -> Result<String> {
        let path = match output {
            Some(x) => format!("/api/random?output={x}"),
            None => format!("/api/random"),
        };

        self.simple_request(reqwest::Method::GET, &path, "").await
    }

    pub async fn random_stream(&self, size: Option<u32>) -> Result<()> {
        let path = match size {
            Some(x) => format!("/api/random/stream?size={x}"),
            None => format!("/api/random/stream"),
        };

        let mut res = self
            .client
            .get(format!("{}{path}", self.base_url))
            .send()
            .await?;

        let mut stdout = std::io::stdout().lock();
        while let Some(chunk) = res.chunk().await? {
            stdout.write_all(&chunk)?;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new("http://localhost:3000");

    let js = "

    const a = () => {
        return 1 + 1
    
    }


    ";

    let mut args = std::env::args();

    // first arg is the name
    args.next();

    match args.next().as_deref() {
        None => {
            println!("{}", client.minify_js(js).await?);
        }
        Some("js") => match args.next().as_deref() {
            Some("minify") => println!("{}", client.minify_js(js).await?),
            Some(x) => eprintln!("invalid command: js {x}"),
            None => eprintln!("missing argument: js"),
        },
        Some("random") => match args.next().as_deref() {
            Some("u32") => println!("{}", client.random(Some(Output::U32)).await?),
            Some("u64") => println!("{}", client.random(Some(Output::U64)).await?),
            Some("u128") => println!("{}", client.random(Some(Output::U128)).await?),
            Some("f32") => println!("{}", client.random(Some(Output::F32)).await?),
            Some("f64") => println!("{}", client.random(Some(Output::F64)).await?),
            Some("bytes") => {
                let size = args.next().map(|x| x.parse().ok()).flatten();
                client.random_stream(size).await?
            },
            Some(x) => eprintln!("invalid argument: random {x}"),
            None => println!("{}", client.random(None).await?),
        },
        Some(x) => {
            dbg!(x);
        }
    }

    Ok(())
}
