use anyhow::Ok;
use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use serde_json::{Value, json};
use tech_notification_v2::application::base::UsecaseTrait;
use tech_notification_v2::application::usecase::random_recommend_usecase::RandomRecommendUsecase;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

async fn handler(_: LambdaEvent<Value>) -> Result<Value, Error> {
    let usecase = RandomRecommendUsecase::new(());
    match usecase.handle().await {
        Ok(()) => Ok(json!({})),
        Err(e) => Ok(json!({ "error": e.to_string() })),
    }
}
