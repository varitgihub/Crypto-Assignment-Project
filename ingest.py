import csv
import json
from kafka import KafkaProducer


producer = KafkaProducer(
    bootstrap_servers="127.0.0.1:9092", 
    value_serializer=lambda v: json.dumps(v).encode("utf-8")
)


with open("trades_data.csv", newline='', encoding='utf-8') as csvfile:
    reader = csv.DictReader(csvfile)
    for row in reader:
        trade = {
            "token_address": row["token_address"],
            "price_in_sol": int(row["price_in_sol"])
        }
        producer.send("trade-data", trade)
        print(f"Produced trade: {trade}")

print("✅ Data sent to Redpanda!")
producer.flush()
producer.close()
