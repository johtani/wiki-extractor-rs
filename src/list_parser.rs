use crate::wiki_page_iterator::{extract_link_text, Document};
use parse_wiki_text::{ListItem, Node};

// TODO need handling order / unorder / definition
// TODO being able to call recursively -Bitward
pub fn parse_items(items: Vec<ListItem>, doc: &mut Document, indent: u8) -> Vec<String> {
    let mut parsed_items = vec![];
    for item in items {
        let mut parsed_item = String::from("* ");
        for node in item.nodes {
            match node {
                Node::Text { value, .. } => {
                    parsed_item.push_str(value);
                }
                Node::CharacterEntity { character, .. } => parsed_item.push(character),
                Node::Link { target, text, .. } => {
                    let link_text = extract_link_text(target, &text);
                    parsed_item.push_str(link_text.clone_text().as_str());
                    doc.links.push(link_text);
                }
                Node::OrderedList { items, .. } => {
                    let inner_items = parse_items(items, doc, indent + 1);
                    for inner_item in inner_items {
                        parsed_item.push_str("\n  ");
                        for i in 0..indent {
                            parsed_item.push_str("  ");
                        }
                        parsed_item.push_str(inner_item.as_str());
                    }
                }
                _ => {}
            }
        }
        parsed_items.push(parsed_item);
    }
    return parsed_items;
}

pub fn parse_order_items(items: Vec<ListItem>, doc: &mut Document, indent: u8) -> Vec<String> {
    let mut parsed_items = vec![];
    let mut index = 1;
    for item in items {
        let mut parsed_item = String::from(format!("{}. ", index));
        index += 1;
        for node in item.nodes {
            match node {
                Node::Text { value, .. } => {
                    parsed_item.push_str(value);
                }
                Node::CharacterEntity { character, .. } => parsed_item.push(character),
                Node::Link { target, text, .. } => {
                    let link_text = extract_link_text(target, &text);
                    parsed_item.push_str(link_text.clone_text().as_str());
                    doc.links.push(link_text);
                }
                Node::UnorderedList { items, .. } => {
                    let inner_items = parse_items(items, doc, indent + 1);
                    for inner_item in inner_items {
                        parsed_item.push_str("\n  ");
                        for i in 0..indent {
                            parsed_item.push_str("  ");
                        }
                        parsed_item.push_str(inner_item.as_str());
                    }
                }
                Node::OrderedList { items, .. } => {
                    let inner_items = parse_order_items(items, doc, indent + 1);
                    for inner_item in inner_items {
                        parsed_item.push_str("\n  ");
                        for i in 0..indent {
                            parsed_item.push_str("  ");
                        }
                        parsed_item.push_str(inner_item.as_str());
                    }
                }
                _ => {}
            }
        }
        parsed_items.push(parsed_item);
    }
    return parsed_items;
}
