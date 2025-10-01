use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use serde_json::{Value, json};
use tech_notification_v2::application::base::UsecaseTrait;
use tech_notification_v2::application::usecase::random_recommend_usecase::{
    RandomRecommendUsecase, RecommendOutput,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

async fn handler(_: LambdaEvent<Value>) -> Result<Value, Error> {
    let usecase = RandomRecommendUsecase::new(());
    let result: anyhow::Result<RecommendOutput> = usecase.handle().await;

    match result {
        Ok(output) => {
            let response = json!({
                "qiita": output.qiita.iter().map(|item| {
                    json!({
                        "title": item.title,
                        "url": item.url,
                        "likes_count": item.likes_count
                    })
                }).collect::<Vec<_>>(),
                "zenn": output.zenn.iter().map(|item| {
                    json!({
                        "title": item.title,
                        "url": item.url,
                        "likes_count": item.likes_count
                    })
                }).collect::<Vec<_>>(),
            });

            Ok(response)
        }
        Err(e) => {
            let error = json!({ "error": e.to_string() });
            Ok(error)
        }
    }
}
