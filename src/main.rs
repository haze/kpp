mod reddit;
use redis::AsyncCommands;
use serenity::prelude::{EventHandler, Mutex, RawEventHandler, RwLock, TypeMap};
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;

const KPP_PICS_KEY: &'static str = "kpp-pics";
const OWNER_ID: u64 = 599131785732816898;

struct Handler;
#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(
        &self,
        ctx: serenity::prelude::Context,
        mut new_message: serenity::model::channel::Message,
    ) {
        if new_message.author.id == OWNER_ID && new_message.content == "kpp" {
            let found_time = std::time::Instant::now();
            if let Some(redis_connection) = ctx.data.read().await.get::<RedisTypemapKey>() {
                let mut conn = redis_connection.lock().await;
                if let Ok(Some(kpop_pic_url)) = random_kpop_pic(&mut conn, Some(100)).await {
                    println!("Found kpop pic after {:?}", found_time.elapsed());
                    if let Err(why) = new_message
                        .edit(&ctx.http, |m| {
                            m.content(kpop_pic_url);
                            m
                        })
                        .await
                    {
                        eprintln!("Failed to edit message: {:?}", &why);
                        return;
                    }
                    println!("Edited message after {:?}", found_time.elapsed());
                }
            }
        }
    }
}
impl RawEventHandler for Handler {}

struct RedisTypemapKey;
impl serenity::prelude::TypeMapKey for RedisTypemapKey {
    type Value = Arc<AsyncMutex<redis::aio::Connection>>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();
    let redis_client =
        redis::Client::open(std::env::var("KPP_REDIS_URL").expect("KPP_REDIS_URL not set"))?;
    let redis_connection = redis_client.get_async_connection().await?;

    let http = serenity::http::client::Http::new_with_token(
        &*std::env::var("KPP_DISCORD_TOKEN").expect("KPP_DISCORD_TOKEN not set"),
    );

    let gateway_url = Arc::new(Mutex::new(http.get_gateway().await?.url));
    let data = Arc::new(RwLock::new(TypeMap::new()));
    {
        let mut data = data.write().await;
        data.insert::<RedisTypemapKey>(Arc::new(AsyncMutex::new(redis_connection)));
    }
    let event_handler = Arc::new(Handler) as Arc<dyn EventHandler>;

    let cache_and_http = Arc::new(serenity::CacheAndHttp {
        cache: Arc::new(serenity::cache::Cache::default()),
        update_cache_timeout: None,
        http: Arc::clone(&Arc::new(http)),
    });

    let (shard_manager, mut shard_manager_worker) = {
        serenity::client::bridge::gateway::ShardManager::new(
            serenity::client::bridge::gateway::ShardManagerOptions {
                data: &data,
                event_handler: &Some(event_handler),
                raw_event_handler: &None,
                shard_index: 0,
                shard_init: 0,
                shard_total: 0,
                ws_url: &gateway_url,
                guild_subscriptions: true,
                cache_and_http: &cache_and_http,
                intents: None,
            },
        )
        .await
    };

    {
        let shard_data = [0, 0, 1];
        let mut manager = shard_manager.lock().await;
        let init = shard_data[1] - shard_data[0] + 1;
        manager.set_shards(shard_data[0], init, shard_data[2]).await;

        if let Err(why) = manager.initialize() {
            manager.shutdown_all().await;
            eprintln!("Shard boot failure: {}", &why);
            return Ok(());
        }
    }

    if let Err(why) = shard_manager_worker.run().await {
        eprintln!("Failed to start: {:?}", &why);
    }

    Ok(())
}

async fn get_reddit_posts(
    subreddit: &str,
    selector: reddit::Selector,
    limit: Option<usize>,
    after: Option<&str>,
) -> Result<reddit::Root, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "https://www.reddit.com/r/{}/{}.json?limit={}&after={}",
        subreddit,
        selector.reddit_name(),
        limit.unwrap_or_else(|| 100),
        after.unwrap_or_default(),
    );
    // println!("{}", &surf::get(&url)
    //     .set_header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.2 Safari/605.1.15")
    //     .recv_string().await?);
    surf::get(url)
        .set_header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.2 Safari/605.1.15")
        .recv_json().await.map_err(|e| e.into())
}

async fn random_kpop_pic(
    redis_connection: &mut redis::aio::Connection,
    max_number_of_pages: Option<usize>,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let start_time = std::time::Instant::now();
    println!("starting to look for kpop image");
    let max_number_of_pages = max_number_of_pages.unwrap_or_else(|| 100);
    let mut pages_searched = 0;
    let mut after: Option<String> = None;

    loop {
        if pages_searched > max_number_of_pages {
            return Ok(None);
        }

        let root =
            get_reddit_posts("kpics", reddit::Selector::New, Some(25), after.as_deref()).await?;
        println!(
            "got reddit page {} data after {:?}",
            pages_searched,
            start_time.elapsed()
        );
        let new_after = root.data.after.clone();

        for random_pic in root.to_set().drain() {
            if !is_already_seen(redis_connection, &random_pic).await? {
                mark_picture_seen(redis_connection, &random_pic).await?;
                return Ok(Some(random_pic.url));
            }
        }

        pages_searched += 1;
        after = Some(new_after);
    }
}

struct DownloadedMedia {
    name: String,
    data: Vec<u8>,
}

async fn is_already_seen(
    redis_connection: &mut redis::aio::Connection,
    post: &reddit::Child,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    redis_connection
        .sismember(KPP_PICS_KEY, &*post.url)
        .await
        .map_err(|e| e.into())
}

async fn mark_picture_seen(
    redis_connection: &mut redis::aio::Connection,
    post: &reddit::Child,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    redis_connection
        .sadd(KPP_PICS_KEY, &*post.url)
        .await
        .map_err(|e| e.into())
}
