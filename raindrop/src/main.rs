pub mod raindrop;

use raindrop::cache::FileItemCache;
use raindrop::client::Client;

fn main() {
    let token = std::env::var("RAINDROP_TOKEN").unwrap();
    let client = Client::new(&token);

    let cache = FileItemCache::default();
    let items = client.get_all_items().unwrap();
    cache.update_cache(items).unwrap();
}
