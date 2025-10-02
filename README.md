Crypto Assignment Project
📌 Overview

This project demonstrates an end-to-end data pipeline using Redpanda (Kafka) and Python services.
It simulates trade data for a crypto token (SOL), streams it into Redpanda, and calculates the Relative Strength Index (RSI) in real-time.

⚙️ Tech Stack

Python 3.12

Kafka (via Redpanda)

Docker & Docker Compose

Redpanda Console (for monitoring topics)

🔄 Workflow

Producer

Reads from a CSV (trades_data.csv) or generates random trades.

Publishes trade data (token_address, price_in_sol) to Redpanda topic: trade-data.

Redpanda Broker

Acts as the message broker between producer and consumer.

RSI Service (Consumer)

Subscribes to the trade-data topic.

Calculates RSI from trade prices.

Logs results in real-time.

🚀 How to Run
1️⃣ Start Redpanda & Console
docker-compose up -d

2️⃣ Start RSI Service (Consumer)
python rsi_service.py

3️⃣ Produce Trades (Choose one)

Option A: Generate random trades (PowerShell loop)

for ($i=1; $i -le 20; $i++) {
    $price = 100 + (Get-Random -Minimum 0 -Maximum 50)
    $json = "{`"token_address`": `"SOL`", `"price_in_sol`": $price}"
    echo $json | docker exec -i redpanda rpk topic produce trade-data
    Start-Sleep -Seconds 1
}


Option B: From CSV

python producer_csv.py

4️⃣ Verify Messages in Redpanda
docker exec -it redpanda rpk topic consume trade-data --brokers redpanda:9092 --offset -5 -n 5

✅ Verification

Trades successfully published into Redpanda.

RSI Service consumes and prints RSI values in real time.

Verified using Redpanda Console (http://localhost:8080
) and message offsets.
