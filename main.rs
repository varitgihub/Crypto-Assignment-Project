use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::config::ClientConfig;
use rdkafka::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct Trade {
    token_address: String,
    price_in_sol: f64,
}

#[derive(Debug, Serialize)]
struct RsiData {
    token_address: String,
    rsi: f64,
}

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "rsi_group")
        .set("bootstrap.servers", "redpanda:9092") // must use redpanda here
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["trade-data"]).expect("Subscription failed");

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "redpanda:9092")
        .create()
        .expect("Producer creation failed");

    let mut price_history: HashMap<String, Vec<f64>> = HashMap::new();

    println!("✅ RSI Service started. Listening for trade-data...");

    while let Ok(message) = consumer.recv().await {
        if let Some(payload) = message.payload() {
            if let Ok(trade) = serde_json::from_slice::<Trade>(payload) {
                let prices = price_history.entry(trade.token_address.clone()).or_default();
                prices.push(trade.price_in_sol);
                if prices.len() > 14 { prices.remove(0); }

                if prices.len() == 14 {
                    let rsi = calculate_rsi(&prices);
                    let rsi_data = RsiData {
                        token_address: trade.token_address.clone(),
                        rsi,
                    };
                    let json = serde_json::to_string(&rsi_data).unwrap();
                    let _ = producer.send(
                        FutureRecord::to("rsi-data")
                            .key(&trade.token_address)
                            .payload(&json),
                        Some(Duration::from_secs(0)),
                    ).await.unwrap();
                    println!("📊 Published RSI for {}: {}", trade.token_address, rsi);
                }
            }
        }
    }
}

fn calculate_rsi(prices: &Vec<f64>) -> f64 {
    let mut gains = 0.0;
    let mut losses = 0.0;
    for i in 1..prices.len() {
        let diff = prices[i] - prices[i - 1];
        if diff > 0.0 { gains += diff; } else { losses -= diff; }
    }
    let avg_gain = gains / 14.0;
    let avg_loss = losses / 14.0;
    if avg_loss == 0.0 { return 100.0; }
    let rs = avg_gain / avg_loss;
    100.0 - (100.0 / (1.0 + rs))
}
