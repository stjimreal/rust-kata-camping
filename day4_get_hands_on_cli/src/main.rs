/*
 * @Date: 2021-09-29 17:27:13
 * @LastEditors: LIULIJING
 * @LastEditTime: 2021-09-30 22:10:56
 */
 
use clap::{AppSettings, Clap};
use colored::Colorize;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};

// 定义 HTTPie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助

/// A native httpie implementation with Rust, can you imagine how easy it is?
#[derive(Clap ,Debug)]
#[clap(version = "1.0", author = "stjimreal <stjimreal@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts{
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 子命令分别对应不同的 HTTP 方法，目前只支持 get / post
#[derive(Clap, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
    // 暂不支持其他方法
}

// get 子命令

/// feed get with an url and we will retrieve the response for you
#[derive(Clap, Debug)]
struct Get{
    /// HTTP 请求 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

fn parse_url(s: &str) -> anyhow::Result<String>{
    // 这里我们仅检查一下 URL 是否合法
    let _url: Url = s.parse()?;

    Ok(s.into())
}

// post 子命令

use std::{collections::HashMap, str::FromStr};
use anyhow::{anyhow, Result};

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Clap, Debug)]
struct Post{
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    /// HTTP 请求的 body
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

/// 命令行中的 key = value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, PartialEq)]
struct KvPair {
    k: String,
    v: String
}

/// 可以用 str.parse() 方法将字符串解析成 KvPair 
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行 split, 这会得到一个迭代器
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为 key, 迭代器返回 Some(T) /None
            // 我们将其转换成 Ok(T)/ Err(E), 然后用 ? 处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为 value
            v: (split.next().ok_or_else(err)?).to_string()
        })
    }
}

/// 因为我们为 KvPair 实现了 FromStr, 这里可以直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

// fn main() {
//     let opts: Opts = Opts::parse();
//     println!("{:?}", opts);
// }


#[tokio::main]
async fn main() -> Result<()> {
    let opts:Opts = Opts::parse();
    // 生成一个 HTTP 客户端
    let client = Client::new();
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}


async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    // println!("{:?}", resp.text().await?);
    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    // println!("{:?}", resp.text().await?);
    Ok(print_resp(resp).await?)
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }

    println!("\n");
}

/// 将服务器返回的 content-type 类型解析成 Mime 类型
fn get_content_type(resp: &Response) -> Option<Mime>{
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

/// 打印服务器返回的 HTTP body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        // 对于 "application/json" 我们 pretty print
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        _ => println!("{}", body),
    }
}

/// 打印整个响应
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    // print_body(mime, &body);
    print_body_sytect(mime, &body);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("https://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }
    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair{
                k: "a".into(),
                v: "1".into()
            }
        );

        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair {
                k: "b".into(),
                v: "".into()
            }
        )
    }
}

use syntect;
use syntect::highlighting::{Style, ThemeSet};
///
/// 使用 syntect 语法高亮继续完善我们的 HTTPie，分别parse `html`, `js`, `json`进行高亮
///
fn print_body_sytect(m: Option<Mime>, body: &String){
    let ss = SyntaxSet::load_defaults_newlines();
    let ts =  ThemeSet::load_defaults();
    let mut syntax = ss.find_syntax_plain_text();
    match m {
        // 对于 "application/json" 我们 pretty print
        Some(v) if v == mime::APPLICATION_JSON  => {
            // println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
            // println!("{}", )
            syntax = ss.find_syntax_by_extension("json").unwrap();
            
        },
        Some(v) if v == mime::TEXT_HTML_UTF_8 || v == mime::TEXT_HTML => {
            // println!("{}", highlighted_html_for_string(body.as_str(), 
            // &ss, ss.find_syntax_plain_text(), &ts.themes["base16-ocean.dark"]))
            syntax = ss.find_syntax_by_extension("html").unwrap();
        },
        Some(v) if v == mime::APPLICATION_JAVASCRIPT 
        || v == mime::TEXT_JAVASCRIPT || v == mime::APPLICATION_JAVASCRIPT_UTF_8 => {
            syntax = ss.find_syntax_by_extension("js").unwrap();
        }
        _ => println!("{}", body),
    }

    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]); 
    for line in LinesWithEndings::from(body) { // LinesWithEndings enables use of newlines mode
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ss);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        println!("{}", escaped);
    }
}