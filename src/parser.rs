use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

// this function parses the xml file and provides the stuff like:
/*
 link
 guid
 guid
 {http://purl.org/dc/elements/1.1/}dc:creator
 {http://purl.org/dc/elements/1.1/}dc:creator
 pubDate
 pubDate
 description
 description
 item
*/

pub fn xml_parser() {
    let file = File::open("test.xml").unwrap();
    let file = BufReader::new(file);


    let test = EventReader::new(file);

    let mut item_track = false;
    let mut link_track = false;
    let mut title_track = false;
    let mut pubdate_track = false;
    //let mut link_num = 0;
    for e in test {
        if item_track {
            match_attr(&e, &mut pubdate_track, String::from("pubDate"));
            match_attr(&e, &mut link_track, String::from("link"));
            match_attr(&e, &mut title_track, String::from("title"));
        }
        // track if we're currently in item
        match_item(&e, &mut item_track);
    }
    //println!("Link count = {}", link_num);
}

pub fn match_item<'a>(stream: &Result<XmlEvent, xml::reader::Error>, item_tracker: &'a mut bool) -> &'a mut bool {
    match &stream {
        Ok(XmlEvent::StartElement { name, .. }) if name.local_name == "item" => {
            *item_tracker = true;
            return item_tracker;
        }
        Ok(XmlEvent::EndElement { name }) if name.local_name == "item" => {
            *item_tracker = false;
            println!();
            return item_tracker;
        }
        Err(stream) => {
            println!("Error in item match: {}", stream);
            return item_tracker;
        }
        _ => {return item_tracker;}
    }
}

pub fn match_attr<'a>(stream: &Result<XmlEvent, xml::reader::Error>, attr_tracker: &'a mut bool, attr: String) -> &'a mut bool {

    match &stream {
        Ok(XmlEvent::StartElement { name, .. }) => {
            if name.local_name == attr {
                *attr_tracker = true;
                //println!("{}", name);
                return attr_tracker;
            } else {
                return attr_tracker;
            }
        }
        // characters = denotes character data outside of tags
        Ok(XmlEvent::Characters(stream)) => {
            if *attr_tracker {
                print!("{}\n", stream);
                return attr_tracker;
            } else {
                return attr_tracker;
            }
        }
        Ok(XmlEvent::EndElement { name }) => {
            if name.local_name == attr {
                *attr_tracker = false;
                return attr_tracker;
            } else {
                return attr_tracker;
            }
        }
        Err(stream) => {
            println!("Error in attr match: {}", stream);
            return attr_tracker;
        }
        _ => {return attr_tracker;}
    }
}