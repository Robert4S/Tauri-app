// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::thread::sleep;
use num_bigint::BigInt;
use num_traits::{One};
use reqwest;
use std::collections::HashMap;

trait Stringify {
    fn to_sci_notation(&self, decimal_stop: usize) -> String;
}

impl Stringify for BigInt {
    fn to_sci_notation(&self, mut decimal_stop: usize) -> String {
        // Only to be used for whole numbers
        let result = self.to_string();
        let mut result = result.chars().collect::<Vec<char>>();
        let result_len = result.len();
        if result_len < 6 {
            let y: String = result.into_iter().collect();
            return y;
        }
        result.insert(1, '.');
        if decimal_stop > result_len {
            decimal_stop = result_len-1;
        }
        let mut result: Vec<char> = result[0..decimal_stop].to_vec();
        result.push('e');
        let mut result: String = result.into_iter().collect();
        result.push_str(&(&result_len-1).to_string());
        result
    }
}
#[tauri::command]
async fn interpost(city: String, temperature: String, question: String, response: String) -> String {
    match temperature.parse::<f32>() {
        Ok(temperature) => {
            postdata(city, temperature, question, response).await.unwrap();
            format!("Data sent to server")
            },
        Err(_) => format!("Invalid temperature"),
    }
}


async fn postdata(city: String, temperature: f32, question: String, response: String) -> Result<reqwest::Response, reqwest::Error>{
    let mut map = HashMap::new();
    map.insert("city", city);
    map.insert("temperature", temperature.to_string());
    map.insert("question", question);
    map.insert("response", response);

    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8000/api/response_creator/")
        .json(&map)
        .send()
        .await?;
    Ok(res)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn calculate_sum(x: &str, y: &str) -> String {
    match x.parse::<f32>() {
        Ok(x2) => match y.parse::<f32>() {
            Ok(y2) => format!("The sum of {} and {} is {}", x2, y2, x2+y2),
            Err(_) => "Invalid input".to_string(),
        },
        Err(_) => "Invalid input".to_string(),
    }
}

fn factorial(n: usize) -> BigInt {
    let mut result = BigInt::one();
    for i in 1..=n {
        result *= i;
    }
    result
}

#[tauri::command]
fn intermediate_factorial(n: &str) -> String {
    match n.parse::<usize>() {
        Ok(n2) => {
            let nfact: BigInt = factorial(n2);
            format!("The factorial of {} is {}", n2, nfact.to_sci_notation(10))
        },
        Err(_) => "Invalid input".to_string(),
    }
}

#[tauri::command]
fn timer(time: &str) -> String {
    let time: u64 = match time.parse() {
        Ok(time) => time,
        Err(_) => return "Invalid time".to_string(),
    };
    sleep(std::time::Duration::from_secs(time));
    format!("{} Second timer finished", time)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            calculate_sum,
            intermediate_factorial,
            timer,
            interpost])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
