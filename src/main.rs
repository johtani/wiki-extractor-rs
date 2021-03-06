use bzip2::read::BzDecoder;
use log::{debug, info, logger, trace, warn};
use parse_wiki_text::{Configuration, Node};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use wiki_extractor::create_ja_config;
use wiki_extractor::output::output_json::OutputJson;
use wiki_extractor::parser::common_parser::{
    extract_external_link_text, extract_heading_text, extract_image, extract_link_text,
};
use wiki_extractor::parser::list_parser::{parse_definition_items, parse_items, parse_order_items};
use wiki_extractor::parser::model::Document;
use wiki_extractor::parser::template_parser::parse_template;
use wiki_extractor::wiki_page_iterator::WikiPageIterator;

fn parse_config(args: &[String]) -> (&str, &str) {
    let input_file = &args[1];
    let output_prefix = &args[2];
    let input_path = Path::new(input_file);
    if input_path.is_file() == false {
        warn!("First argument should be file path.");
        panic!("First argument should be file path.");
    }
    (input_file, output_prefix)
}
fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    let (path, output_path) = parse_config(&args);

    parse_wiki(path, output_path);

    info!("Finish wiki-extractor. ");
}

fn parse_wiki(path: &str, output_path: &str) {
    let file = File::open(path).unwrap();
    let buf = BzDecoder::new(file);
    //let buf = BufReader::new(file);
    let mut output = OutputJson::new(output_path, 10000);

    //TODO need a flag to skip Wikipedia special page that starts "Wikipedia:" in title.
    let _xml_parser = WikiPageIterator::new(buf);

    for page in _xml_parser {
        // TODO should support output meta flag?
        if !page.meta {
            info!(
                "Id[{}] - Title:[{}] - Timestamp:[{}] - meta?:[{}]",
                page.id, page.title, page.timestamp, page.meta
            );

            let result = create_ja_config().parse(page.raw_content.as_str());
            let mut page_content = String::new();
            let mut doc: Document = Document {
                id: page.id.to_string(),
                title: page.title.to_string(),
                timestamp: page.timestamp.to_string(),
                revision_id: page.revision_id.to_string(),
                url: format!(
                    "https://ja.wikipedia.org/wiki?curid={}",
                    page.id.to_string()
                ),
                contents: vec![],
                categories: vec![],
                headings: vec![],
                images: vec![],
                links: vec![],
                redirect_to: None,
            };

            for node in result.nodes {
                //match node {}
                match node {
                    Node::Category { target, .. } => doc
                        .categories
                        .push(target.replace("Category:", "").trim().to_string()),

                    Node::CharacterEntity { character, .. } => page_content.push(character),

                    Node::Text { value, .. } => {
                        if value != "\n" {
                            page_content.push_str(value)
                        }
                    }
                    Node::Heading { nodes, .. } => {
                        doc.contents.push(page_content.to_string());
                        page_content.clear();
                        let heading = extract_heading_text(&nodes);
                        add_heading(heading.as_str(), &mut page_content, &mut doc);
                    }
                    Node::ExternalLink { nodes, .. } => {
                        let link_text = extract_external_link_text(&nodes);
                        page_content.push_str(link_text.clone_text().as_str());
                        doc.links.push(link_text);
                    }
                    Node::Image { target, text, .. } => {
                        // need to parse recursive in text
                        doc.images.push(extract_image(target, &text));
                    }
                    Node::Link { target, text, .. } => {
                        let link_text = extract_link_text(target, &text);
                        page_content.push_str(link_text.clone_text().as_str());
                        doc.links.push(link_text);
                    }

                    Node::UnorderedList { items, .. } => {
                        for parsed_item in parse_items(items, &mut doc, 1) {
                            page_content.push_str("\n  ");
                            page_content.push_str(parsed_item.as_str());
                        }
                    }
                    Node::OrderedList { items, .. } => {
                        for parsed_item in parse_order_items(items, &mut doc, 1) {
                            page_content.push_str("\n  ");
                            page_content.push_str(parsed_item.as_str());
                        }
                    }
                    Node::DefinitionList { items, .. } => {
                        for parsed_item in parse_definition_items(items, &mut doc, 1) {
                            page_content.push_str("\n  ");
                            page_content.push_str(parsed_item.as_str());
                        }
                    }
                    // TODO contentをパラグラフごとに分割するなら、ここで区切る?
                    Node::ParagraphBreak { .. } => {
                        page_content.push_str("\n");
                    }
                    Node::Template {
                        name, parameters, ..
                    } => {
                        if let Some(template) = parse_template(&name, &parameters) {
                            page_content.push_str(template.as_str());
                        }
                    }
                    Node::Redirect { target, .. } => {
                        doc.redirect_to = Some(target.to_string());
                    }

                    // // TODO Need extracte cells
                    // Node::Table { .. } => {}
                    //
                    // Node::Tag { .. } => {}
                    //
                    // // TODO template combination?

                    // Node::Parameter { .. } => {}
                    // //TODO
                    // Node::HorizontalDivider { .. } => {}
                    // Node::MagicWord { .. } => {}
                    // Node::Preformatted { .. } => {}
                    //
                    // TODO maybe NO-OP
                    Node::StartTag { .. } => {}
                    Node::EndTag { .. } => {}
                    //NO-OP
                    Node::Bold { .. } => {}
                    Node::BoldItalic { .. } => {}
                    Node::Italic { .. } => {}
                    Node::Comment { .. } => {}
                    _ => {
                        trace!("あ    {:?}", node);
                    }
                }
            }

            if result.warnings.is_empty() == false {
                for warning in result.warnings {
                    debug!(
                        "[WARN] {} start:{} - end:{}",
                        warning.message, warning.start, warning.end
                    );
                }
            }

            if page_content.is_empty() == false && doc.contents.len() > 0 {
                doc.contents.push(page_content.to_string());
            }
            output.output(&doc);
        //print_doc(&doc);
        } else {
            info!(
                "Skip : Id[{}] - Title:[{}] - Timestamp:[{}] - meta?:[{}]",
                page.id, page.title, page.timestamp, page.meta
            );
        }
    }
    output.flush();
}

// for test
fn print_doc(doc: &Document) {
    trace!(
        "# of sections & contents. [{}] = [{}]",
        doc.headings.len(),
        doc.contents.len()
    );
    trace!("Page::id  {}", doc.id);
    trace!("Page::title  {}", doc.title);
    trace!("Content \n{}", doc.contents.join("\n"));
    trace!("Categories \n{}", doc.categories.join("\n"));
    //debug!("Images {:?}", doc.images);
    //debug!("Links {:?}", doc.links)
}

fn add_heading(heading: &str, page_content: &mut String, doc: &mut Document) {
    doc.headings.push(heading.to_string());
    page_content.push_str(heading);
    page_content.push('\n');
}
