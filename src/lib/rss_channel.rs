use std::vec;
use super::rss_item::rss_item;

pub struct rss_channel {
    items: Vec<rss_item>,
    title: String,
    link: String,
    description: String,
}
