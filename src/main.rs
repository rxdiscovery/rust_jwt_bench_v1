use futures::stream::FuturesUnordered;
use futures::StreamExt;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: u128,
    exp: u128,
}

#[tokio::main]
async fn main() {
    if true {
        multi_thread_v2().await;
    } else {
        mono_thread()
    }
}

async fn multi_thread_v2() {
    println!("=== Rust bench ===");

    let total_cpus = num_cpus::get();

    let nbr_cpus = total_cpus - 2;
    println!("total cpus : {}", total_cpus);
    println!("use only : {} cpus", nbr_cpus);
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(nbr_cpus)
        .build()
        .expect("runtime error");

    let args: Vec<String> = env::args().collect();
    let file_contents = fs::read_to_string("./emails.json").unwrap();
    let emails: Vec<String> = serde_json::from_str(&file_contents).unwrap();

    let jwt_secret = "private_key.pem";
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let jwt_decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let num_iterations = args[1].parse::<u64>().unwrap();
    let start_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    let validation = Validation::new(Algorithm::HS256);

    let total_iter = Arc::new(AtomicU64::new(0u64));

    let mut futures = FuturesUnordered::new();

    for _t in 0..nbr_cpus {
        futures.push({
            let jwt_encoding_key = jwt_encoding_key.clone();
            let jwt_decoding_key = jwt_decoding_key.clone();
            let validation = validation.clone();
            let emails = emails.clone();
            let total_iter = total_iter.clone();

            runtime.spawn(async move {
                let mut idx = 0;

                loop {
                    let i = total_iter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                    let email = &emails[idx];
                    idx += 1;
                    let curr_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
                    let my_claims = Claims {
                        sub: email.to_string(),
                        iat: curr_ts,
                        exp: curr_ts + 2 * 60 * 60 * 1000,
                    };

                    let token = encode(&Header::default(), &my_claims, &jwt_encoding_key).unwrap();

                    //println!("Token: {}", token);
                    let token_data =
                        decode::<Claims>(&token, &jwt_decoding_key, &validation).unwrap();

                    if token_data.claims.sub != *email {
                        panic!("email didn't match");
                    }
                    if idx >= emails.len() {
                        idx = 0;
                    }

                    if i >= num_iterations {
                        break;
                    }
                }
            })
        });
    }

    let _ = (futures.next().await).unwrap();

    let end_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    let diff = end_ts - start_ts;

    println!(
        "Total iterations : {} , endend in : {} ms",
        total_iter.load(std::sync::atomic::Ordering::SeqCst),
        diff
    );

    runtime.shutdown_background();
}

#[allow(dead_code)]
async fn multi_thread() {
    let total_cpus = num_cpus::get();

    let nbr_cpus = total_cpus - 12;
    println!("total cpus : {}", total_cpus);
    println!("use only : {} cpus", nbr_cpus);
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(nbr_cpus)
        .build()
        .expect("runtime error");

    let args: Vec<String> = env::args().collect();
    let file_contents = fs::read_to_string("./emails.json").unwrap();
    let emails: Vec<String> = serde_json::from_str(&file_contents).unwrap();

    let jwt_secret = "private_key.pem";
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let jwt_decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let num_iterations = args[1].parse::<u64>().unwrap();
    let start_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    let validation = Validation::new(Algorithm::HS256);

    let total_iter = Arc::new(AtomicU64::new(0u64));

    let tasks = (0..nbr_cpus)
        .map(|_i| {
            let jwt_encoding_key = jwt_encoding_key.clone();
            let jwt_decoding_key = jwt_decoding_key.clone();
            let validation = validation.clone();
            let emails = emails.clone();
            let total_iter = total_iter.clone();

            runtime.spawn(async move {
                let mut idx = 0;

                loop {
                    let i = total_iter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                    let email = &emails[idx];
                    idx += 1;
                    let curr_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
                    let my_claims = Claims {
                        sub: email.to_string(),
                        iat: curr_ts,
                        exp: curr_ts + 2 * 60 * 60 * 1000,
                    };

                    let token = encode(&Header::default(), &my_claims, &jwt_encoding_key).unwrap();

                    //println!("Token: {}", token);
                    let token_data =
                        decode::<Claims>(&token, &jwt_decoding_key, &validation).unwrap();

                    if token_data.claims.sub != *email {
                        panic!("email didn't match");
                    }
                    if idx >= emails.len() {
                        idx = 0;
                    }

                    if i >= num_iterations {
                        break;
                    }
                }
            })
        })
        .collect::<FuturesUnordered<_>>();

    let _result = futures::future::join_all(tasks).await;

    let end_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    let diff = end_ts - start_ts;

    println!(
        "Total iterations : {} , endend in : {} ms",
        total_iter.load(std::sync::atomic::Ordering::SeqCst),
        diff
    );

    runtime.shutdown_background();
}

fn mono_thread() {
    let args: Vec<String> = env::args().collect();
    let file_contents = fs::read_to_string("./emails.json").unwrap();
    let emails: Vec<String> = serde_json::from_str(&file_contents).unwrap();

    let mut i = 1;
    let mut idx = 0;
    let jwt_secret = "private_key.pem";
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let jwt_decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let num_iterations = args[1].parse::<i32>().unwrap();
    let start_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    let validation = Validation::new(Algorithm::HS256);

    loop {
        let email = &emails[idx];
        idx += 1;
        let curr_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
        let my_claims = Claims {
            sub: email.to_string(),
            iat: curr_ts,
            exp: curr_ts + 2 * 60 * 60 * 1000,
        };

        let token = encode(&Header::default(), &my_claims, &jwt_encoding_key).unwrap();

        //println!("Token: {}", token);
        let token_data = decode::<Claims>(&token, &jwt_decoding_key, &validation).unwrap();
        if token_data.claims.sub != *email {
            panic!("email didn't match");
        }
        if idx >= emails.len() {
            idx = 0;
        }
        i += 1;
        if i > num_iterations {
            break;
        }
    }

    let end_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    let diff = end_ts - start_ts;
    println!("{}", diff);
}
