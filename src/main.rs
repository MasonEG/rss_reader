use std::env;
use std::io::BufReader;
use std::collections::HashMap;
use xml::{
    reader::{EventReader, XmlEvent},
    name::OwnedName,
    attribute::OwnedAttribute,
};
use termion::{
    event::Key,
    input::TermRead,
    color,
    raw::IntoRawMode,
};

mod lib;
use lib::rss_item::rss_item;

enum ReaderState {
    title,
    description,
    date,
    url,
    author,
    summary,
    irrelevant,
}

// probably useless
fn parse_element(name: OwnedName, attr: Vec<OwnedAttribute>, depth: usize) {
    match name.local_name.as_str() {
        "item" => {
            println!("item");
        }
        "description" => {
            println!("description");
        },
        _ => {} // everything else
        
    }    
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let args: Vec<String> = env::args().collect();
    println!("RSS feed from: {}", args.get(1)?);


    let resp = reqwest::blocking::get(args.get(1)?)? // ny times rss: https://feeds.simplecast.com/54nAGcIl
    .text()?;

    let xml_parser = EventReader::from_str(resp.as_str());
    let mut state = ReaderState::irrelevant;
    let mut items: Vec<rss_item> = Vec::new();
    let mut reading_item: bool = false;    
    let mut title = String::new();
    let mut description = String::new();
    let mut date = String::new();
    let mut url = String::new();

    // main parse loop, where the chaos happens
    for e in xml_parser {
        match e {
            // start of tag
            Ok(XmlEvent::StartElement { name, .. }) => {
                //println!("---Start: {} at depth: {}", name, depth);

                // find state
                match name.local_name.as_str() {
                    "title" => { state = ReaderState::title; }
                    "description" => { state = ReaderState::description; }
                    "pubDate" => { state = ReaderState::date; }
                    "link" => { state = ReaderState::url; }
                    "item" => { reading_item = true;}
                    _ => { state = ReaderState::irrelevant; }
                }

                // println!("tag name: {}", name.local_name.as_str()); // debug

                // print the state for testing
                let reader_state_str = match state {
                    ReaderState::title => String::from("title"),
                    ReaderState::description => String::from("description"),
                    ReaderState::date => String::from("date"),
                    ReaderState::url => String::from("url"),
                    ReaderState::author => String::from("author"),
                    ReaderState::summary => String::from("summary"),
                    ReaderState::irrelevant => String::from("irrelevant")
                };
                 // println!("reader state: {}", reader_state_str);
            }

            // end of tag
            Ok(XmlEvent::EndElement { name, .. }) => {
                state = ReaderState::irrelevant;
                if (name.local_name.as_str() == "item") {
                    reading_item = false;
                    items.push(rss_item{title: title.clone(), date: date.clone(), description: description.clone(), url: url.clone()});
                }
            }

            // text content inside tag
            Ok(XmlEvent::Characters(s)) => {
                // println!("characters event");
                if reading_item {
                    match state {
                        ReaderState::title => {
                            title = s.clone();
                            // println!("content: {}", s);
                        }
                        ReaderState::description => {
                            description = s.clone();
                            // println!("content: {}", s);
                        }
                        ReaderState::url => {
                            url = s.clone();
                            // println!("content: {}", s);
                        }
                        ReaderState::date => {
                            date = s.clone();
                        }
                        ReaderState::author => {

                        }
                        ReaderState::summary => {

                        }
                        ReaderState::irrelevant => {}
                    }
                }
            }

            Ok(XmlEvent::CData(s)) => {
                match state {
                    ReaderState::description => description = s.clone(),
                    _ => {}
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        } // end xml event match
    } // end for


    for i in items {
        println!("{}", i);
    }


    //println!("Response: {:?}", resp);
    Ok(())
}
