import json
from kafka import KafkaConsumer

def calculate_rsi(prices, period=14):
    if len(prices) < period:
        return None
    deltas = [prices[i+1] - prices[i] for i in range(len(prices)-1)]
    gains = [d for d in deltas if d > 0]
    losses = [-d for d in deltas if d < 0]

    avg_gain = sum(gains)/period if gains else 0
    avg_loss = sum(losses)/period if losses else 0

    if avg_loss == 0:
        return 100

    rs = avg_gain / avg_loss
    return 100 - (100 / (1 + rs))

# connect to Redpanda
consumer = KafkaConsumer(
    "trade-data",
    bootstrap_servers="127.0.0.1:19092",
    value_deserializer=lambda v: json.loads(v.decode("utf-8")),
    auto_offset_reset="latest",
    group_id="rsi_demo"
)

print("✅ RSI Service started. Listening for trade-data...")

prices = []
for message in consumer:
    trade = message.value
    prices.append(trade["price_in_sol"])
    rsi = calculate_rsi(prices)
    if rsi:
        print(f"📊 Published RSI for {trade['token_address']}: {rsi}")
