use crate::parser::common_parser::{extract_external_link_text, extract_link_text};
use crate::parser::model::Document;
use crate::parser::template_parser::parse_template;
use log::trace;
use parse_wiki_text::{DefinitionListItem, DefinitionListItemType, ListItem, Node};

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
                Node::ExternalLink { nodes, .. } => {
                    let link_text = extract_external_link_text(&nodes);
                    parsed_item.push_str(link_text.clone_text().as_str());
                    doc.links.push(link_text);
                }
                Node::UnorderedList { items, .. } => {
                    let inner_items = parse_items(items, doc, indent + 1);
                    indent_items(indent, &mut parsed_item, inner_items);
                }
                Node::OrderedList { items, .. } => {
                    let inner_items = parse_order_items(items, doc, indent + 1);
                    indent_items(indent, &mut parsed_item, inner_items);
                }
                Node::DefinitionList { items, .. } => {
                    let inner_items = parse_definition_items(items, doc, indent + 1);
                    indent_items(indent, &mut parsed_item, inner_items);
                }
                Node::Template {
                    name, parameters, ..
                } => {
                    if let Some(template) = parse_template(&name, &parameters) {
                        parsed_item.push_str(template.as_str());
                    }
                }
                // TODO maybe NO-OP
                Node::StartTag { .. } => {}
                Node::EndTag { .. } => {}
                //NO-OP
                Node::Bold { .. } => {}
                Node::BoldItalic { .. } => {}
                Node::Italic { .. } => {}
                Node::Comment { .. } => {}
                _ => {
                    trace!("ぶ    {:?}", node);
                }
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
                    indent_items(indent, &mut parsed_item, inner_items)
                }
                Node::OrderedList { items, .. } => {
                    let inner_items = parse_order_items(items, doc, indent + 1);
                    indent_items(indent, &mut parsed_item, inner_items);
                }
                Node::DefinitionList { items, .. } => {
                    let inner_items = parse_definition_items(items, doc, indent + 1);
                    indent_items(indent, &mut parsed_item, inner_items);
                }
                Node::Template {
                    name, parameters, ..
                } => {
                    if let Some(template) = parse_template(&name, &parameters) {
                        parsed_item.push_str(template.as_str());
                    }
                }
                // TODO maybe NO-OP
                Node::StartTag { .. } => {}
                Node::EndTag { .. } => {}
                //NO-OP
                Node::Bold { .. } => {}
                Node::BoldItalic { .. } => {}
                Node::Italic { .. } => {}
                Node::Comment { .. } => {}
                _ => {
                    trace!("り    {:?}", node);
                }
            }
        }
        parsed_items.push(parsed_item);
    }
    return parsed_items;
}

fn indent_items(indent: u8, parsed_item: &mut String, inner_items: Vec<String>) {
    for inner_item in inner_items {
        parsed_item.push_str("\n  ");
        for _i in 0..indent {
            parsed_item.push_str("  ");
        }
        parsed_item.push_str(inner_item.as_str());
    }
}

pub fn parse_definition_items(
    items: Vec<DefinitionListItem>,
    doc: &mut Document,
    indent: u8,
) -> Vec<String> {
    let mut parsed_items = vec![];
    for item in items {
        let mut parsed_item = String::new();
        if let DefinitionListItemType::Details = item.type_ {
            parsed_item.push_str("  ");
        }
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
                Node::ExternalLink { nodes, .. } => {
                    let link_text = extract_external_link_text(&nodes);
                    parsed_item.push_str(link_text.clone_text().as_str());
                    doc.links.push(link_text);
                }
                Node::UnorderedList { items, .. } => {
                    let inner_items = parse_items(items, doc, indent + 1);
                    indent_items(indent, &mut parsed_item, inner_items);
                }
                Node::OrderedList { items, .. } => {
                    let inner_items = parse_order_items(items, doc, indent + 1);
                    indent_items(indent, &mut parsed_item, inner_items);
                }
                Node::DefinitionList { items, .. } => {
                    let inner_items = parse_definition_items(items, doc, indent + 1);
                    indent_items(indent, &mut parsed_item, inner_items);
                }
                Node::Template {
                    name, parameters, ..
                } => {
                    if let Some(template) = parse_template(&name, &parameters) {
                        parsed_item.push_str(template.as_str());
                    }
                }
                // TODO maybe NO-OP
                Node::StartTag { .. } => {}
                Node::EndTag { .. } => {}
                //NO-OP
                Node::Bold { .. } => {}
                Node::BoldItalic { .. } => {}
                Node::Italic { .. } => {}
                Node::Comment { .. } => {}
                _ => {
                    trace!("で    {:?}", node);
                }
            }
        }
        parsed_items.push(parsed_item);
    }
    return parsed_items;
}
