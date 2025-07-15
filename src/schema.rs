use chrono::{DateTime, Utc};
use juniper::{EmptySubscription, FieldResult, RootNode};
use juniper::{GraphQLInputObject, GraphQLObject};
use questdb::ingress::{
    Sender,
    Buffer,
    TimestampMicros,
    TimestampNanos};
use sqlx::{PgPool, FromRow};
use std::env;
    
#[derive(GraphQLObject, FromRow)]
#[graphql(description = "A SCD41 measurement")]
struct Measurement {
    battery: f64,
    co2: i32,
    humidity : f64,
    temperature: f64,
    device: String,
    ts: String
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A SCD41 measurement")]
struct NewMeasurement {
    battery: f64,
    co2: i32,
    humidity : f64,
    temperature: f64,
    device: String,
    ts: String
}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn measurement(context: &Context, _device: String) -> FieldResult<Vec<Measurement>> {
        let pool = &context.pool;
        let rows = sqlx::query_as::<_, Measurement>("SELECT battery, co2, humidity, temperature, device, ts::TEXT as ts FROM measurements WHERE device = $1")
            .bind(_device)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    fn create_measurement(_context: &Context, new_measurement: NewMeasurement) -> FieldResult<Measurement> {
        let mut sender = Sender::from_conf(env::var("DATABASE_HTTP_URL").unwrap_or_else(|_| "http::addr=questdb:9000;protocol_version=2;".to_string()))?;
        let mut buffer = Buffer::new(questdb::ingress::ProtocolVersion::V2);

        let dt: DateTime<Utc> =  DateTime::parse_from_rfc3339(&new_measurement.ts).expect("Failed to parse datetime string").with_timezone(&Utc);

        buffer
            .table("measurements")?
            .symbol("device", new_measurement.device.to_owned())?
            .column_f64("battery", new_measurement.battery)?
            .column_i64("co2", i64::from(new_measurement.co2) )?
            .column_f64("humidity", new_measurement.humidity)?
            .column_f64("temperature", new_measurement.temperature)?
            .column_ts("ts", TimestampMicros::new(dt.timestamp_micros()))?
            // .column_ts("ts", dt.timestamp_nanos_opt())?
            .at(TimestampNanos::now())?;
        sender.flush(&mut buffer)?;
        
        Ok(Measurement {
            device: new_measurement.device,
            battery: new_measurement.battery,
            co2: new_measurement.co2,
            humidity : new_measurement.humidity,
            temperature: new_measurement.temperature,
            ts: new_measurement.ts,
        })
    }
}

pub struct Context {
    pub pool: PgPool
}
impl juniper::Context for Context {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(
        QueryRoot,
        MutationRoot,
        EmptySubscription::<Context>::new(),
    )
}