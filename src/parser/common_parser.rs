use crate::parser::model::{Image, ImageType, Link, Text};
use parse_wiki_text::Node;
use std::str::FromStr;

// extract heading text
pub fn extract_heading_text(nodes: &Vec<Node>) -> String {
    let mut text = String::new();
    for node in nodes {
        match node {
            Node::Text { value, .. } => text.push_str(value),
            // TODO check other patterns
            _ => {}
        }
    }
    return text;
}

// only return text.
pub fn parse_text_only(nodes: &Vec<Node>) -> String {
    let mut str = String::new();
    for node in nodes {
        match node {
            Node::Text { value, .. } => str.push_str(value),
            // should we care more pattern?
            _ => {}
        }
    }
    return str;
}

// if nodes has Link node, return text with link info
pub fn parse_text(nodes: &Vec<Node>) -> Text {
    let mut str = String::new();
    let mut link_str = String::new();
    for node in nodes {
        match node {
            Node::Text { value, .. } => str.push_str(value),
            Node::Link { target, text, .. } => {
                link_str.push_str(target);
                str.push_str(parse_text_only(text).as_str());
            }
            // Is there
            _ => {}
        }
    }
    return if link_str.len() > 0 {
        Text::LinkText {
            text: str,
            link: Link::Link { target: link_str },
        }
    } else {
        Text::Text { text: str }
    };
}

// parse image tag info
pub fn extract_image(target: &str, text: &Vec<Node>) -> Image {
    let targets: Vec<String> = target.split(":").map(|e| e.to_string()).collect();
    if targets.len() != 2 {
        panic!(format!("Not expected Image data... {} ", target));
    }

    let image = Image {
        target: targets[1].to_string(),
        target_type: ImageType::from_str(targets[0].as_str()).unwrap(),
        text: parse_text(text),
    };
    //println!("い {:?}", image);
    return image;
}

// parse link text
pub fn extract_link_text(target: &str, nodes: &Vec<Node>) -> Text {
    return Text::LinkText {
        text: parse_text_only(nodes),
        link: Link::Link {
            target: target.to_string(),
        },
    };
}

// parse external link text
pub fn extract_external_link_text(nodes: &Vec<Node>) -> Text {
    if nodes.len() > 1 {
        panic!(format!("nodes.len() > 1. unexpected data. {:?}", nodes));
    } else {
        let text = parse_text_only(nodes);
        let link_text: Vec<&str> = text.splitn(2, " ").collect();
        if link_text.len() == 1 {
            return Text::LinkText {
                text: link_text[0].to_string(),
                link: Link::ExternalLink {
                    target: link_text[0].to_string(),
                },
            };
        } else {
            return Text::LinkText {
                text: link_text[1].to_string(),
                link: Link::ExternalLink {
                    target: link_text[0].to_string(),
                },
            };
        }
    }
}
