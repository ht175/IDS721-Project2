use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
//import the random fruit function from the lib.rs file
// use webdocker::random_fruit;

//create a function that returns a hello world
use exitfailure::ExitFailure;

use reqwest::{Url};
use select::document::Document;
use std::io::Cursor;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
 struct CompanyQuote {
    c: f64,
    h: f64,
    l: f64,
    o: f64,
    pc: f64,
    t: i128,
}

impl CompanyQuote {
    async fn new(symbol: &String, api_key: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<CompanyQuote>().await?;

        Ok(res)
    }
}

#[get("/currprice/{name}")]
async fn currprice(name: web::Path<String>) -> impl Responder {
    let api_key = "cft728hr01qokdd044q0cft728hr01qokdd044qg".to_string();
    let res = CompanyQuote::new(&name, &api_key).await.unwrap();
    println!("{}'s current stock price: {}", name, res.c);
    HttpResponse::Ok().body(res.c.to_string()
 
    )
}

#[get("/highprice/{name}")]
async fn highprice(name: web::Path<String>) -> impl Responder {
    let api_key = "cft728hr01qokdd044q0cft728hr01qokdd044qg".to_string();
    let res = CompanyQuote::new(&name, &api_key).await.unwrap();
    println!("{}'s high stock price: {}", name, res.h);
    HttpResponse::Ok().body(res.h.to_string())
}

#[get("/lowprice/{name}")]
async fn lowprice(name: web::Path<String>) -> impl Responder {
    let api_key = "cft728hr01qokdd044q0cft728hr01qokdd044qg".to_string();
    let res = CompanyQuote::new(&name, &api_key).await.unwrap();
    println!("{}'s low stock price: {}", name, res.l);
    HttpResponse::Ok().body(
        res.l.to_string()
 
    )
}

#[get("/openprice/{name}")]
async fn openprice(name: web::Path<String>) -> impl Responder {
    let api_key = "cft728hr01qokdd044q0cft728hr01qokdd044qg".to_string();
    let res = CompanyQuote::new(&name, &api_key).await.unwrap();
    println!("{}'s open stock price: {}", name, res.o);
    HttpResponse::Ok().body(
        res.o.to_string()
    )
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(
        "<html>
    <head>
        <title>Hello, This is a  web server for checking stock data</title>
    </head>
    <body>
        <p>you can get close price of stock you are interested in</p>
        <p>you can get high prices of stock you are interested in</p>
        <p>you can get low prices of stock you are interested in</p> 
        <p>you can get open prices of stock you are interested in</p>
    </body>
</html>",
    )
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(currprice)
            .service(highprice)
            .service(lowprice)
            .service(openprice)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
