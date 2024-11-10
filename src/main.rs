mod request_data;
mod link_finder;

use crate::link_finder::link_finder;
use request_data::request_data;
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;

const NUM_REQUESTS : usize = 10;

#[tokio::main]
async fn main() {
    let unvisited_links = Arc::new(Mutex::new(BTreeSet::from(["https://ru.wikipedia.org".to_string()])));
    let visited_links = Arc::new(Mutex::new(BTreeSet::new()));

    let semaphore = Arc::new(Semaphore::new(NUM_REQUESTS));
    let mut handles = Vec::new();

    loop {
        let permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();

        let unvisited = Arc::clone(&unvisited_links);
        let visited = Arc::clone(&visited_links);
        let handle = tokio::spawn(async move {
            let _permit = permit;
            let mut unvisited_new = unvisited.lock().unwrap();
            let current_link: String = unvisited_new.pop_first().unwrap();
            drop(unvisited_new);

            println!("{:#?}", current_link);

            let body = request_data(&current_link).await;

            let links_array = link_finder(&body);
            let mut unvisited_new = unvisited.lock().unwrap();
            let mut visited_new = visited.lock().unwrap();
            for link in links_array {
                if visited_new.get(&link).is_none() && link != current_link {
                    unvisited_new.insert(link);
                }
            }

            visited_new.insert(current_link);

            println!("{:#?}", visited_new.len());
            println!("{:#?}", unvisited_new.len());
        });
        handles.push(handle);

    }
}
