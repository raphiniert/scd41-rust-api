# SCD41-RUST-API

A Rust GraphQL API using [Actix Web](https://actix.rs/), [Juniper](https://github.com/graphql-rust/juniper) and [SQLx](https://github.com/launchbadge/sqlx/blob/main/README.md).

## API

```env
# questdb
DATABASE_HTTP_URL=http::addr=questdb:9000;
DATABASE_POSTGRES_URL=postgres://admin:quest@questdb:8812/qdb
DATABASE_MAX_CONNECTIONS=5
```

## QUESTDB

```sql
CREATE TABLE measurements (
  battery DOUBLE,
  co2 INT,
  humidity DOUBLE,
  temperature DOUBLE,
  device SYMBOL,
  ts TIMESTAMP
) TIMESTAMP(ts) PARTITION BY DAY WAL
DEDUP UPSERT KEYS(ts, device);
```

```sql
SELECT * from measurements;
```

### Query
```graphql
{
  measurement(device: "1312") {
    device
    battery
    temperature
  } 
}
```

```graphql
mutation CreateMeasurementEntry($m: NewMeasurement!) {
  createMeasurement(newMeasurement: $m) {
    device
  }
}
```
```graphql
{
  "m": {
  	"device": "1312",
    "battery": 90.0,
    "co2": 1000,
    "humidity": 50.0,
    "temperature": 20.0,
    "ts": "2024-01-01T01:23:45.7+01:00"
  }
}
```