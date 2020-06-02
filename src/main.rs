use parse_wiki_text::{Configuration, Node};
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use wiki_extractor::list_parser::{parse_items, parse_order_items};
use wiki_extractor::template_parser::{
    get_lang_template_text, get_temporary_link_template_text, parse_template, parse_template_type,
    TemplateType,
};
use wiki_extractor::wiki_page_iterator::{
    extract_heading_text, extract_image, extract_link_text, parse_text_only, Document, Link,
};
use wiki_extractor::wiki_page_iterator::{Image, ImageType, Page, Text, WikiPageIterator};

fn main() {
    let path = "/Users/johtani/tmp/wiki/sample.xml";
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);

    //TODO need a flag to skip Wikipedia special page that starts "Wikipedia:" in title.
    let _xml_parser = WikiPageIterator::new(buf);

    for page in _xml_parser {
        if !page.meta {
            println!(
                "Id[{}] - Title:[{}] - Timestamp:[{}] - meta?:[{}]",
                page.id, page.title, page.timestamp, page.meta
            );

            let result = Configuration::default().parse(page.raw_content.as_str());
            let mut page_content = String::new();
            let mut doc: Document = Document {
                page: &page,
                contents: vec![],
                categories: vec![],
                headings: vec![],
                images: vec![],
                links: vec![],
            };
            add_heading(&page.title, &mut page_content, &mut doc);

            for node in result.nodes {
                //match node {}
                println!("あ    {:?}", node);
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

                    Node::ExternalLink { .. } => {
                        //TODO
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
                    Node::DefinitionList { .. } => {
                        println!("あ    {:?}", node);
                    }
                    // TODO contentをパラグラフごとに分割するなら、ここで区切る?
                    Node::ParagraphBreak { .. } => {
                        page_content.push_str("\n");
                    }

                    // TODO Need extracte cells
                    Node::Table { .. } => {}

                    Node::Tag { .. } => {}

                    // TODO template combination?
                    Node::Template {
                        name, parameters, ..
                    } => {
                        if let Some(template) = parse_template(&name, &parameters) {
                            page_content.push_str(template.as_str());
                        }
                    }
                    Node::Redirect { .. } => {}
                    Node::Parameter { .. } => {}
                    //TODO
                    Node::HorizontalDivider { .. } => {}
                    Node::MagicWord { .. } => {}
                    Node::Preformatted { .. } => {}

                    // TODO maybe NO-OP
                    Node::StartTag { .. } => {}
                    Node::EndTag { .. } => {}

                    //NO-OP
                    Node::Bold { .. } => {}
                    Node::BoldItalic { .. } => {}
                    Node::Italic { .. } => {}
                    Node::Comment { .. } => {}
                }
            }

            if result.warnings.is_empty() == false {
                for warning in result.warnings {
                    println!(
                        "い    [WARN] {} start:{} - end:{}",
                        warning.message, warning.start, warning.end
                    );
                }
            }

            doc.contents.push(page_content.to_string());
            print_doc(doc);
        } else {
            println!(
                "Skip : Id[{}] - Title:[{}] - Timestamp:[{}] - meta?:[{}]",
                page.id, page.title, page.timestamp, page.meta
            );
        }
    }

    fn print_doc(doc: Document) {
        println!(
            "# of sections & contents. [{}] = [{}]",
            doc.headings.len(),
            doc.contents.len()
        );
        println!("Page::id  {}", doc.page.id);
        println!("Page::title  {}", doc.page.title);
        println!("Content\n {}", doc.contents.join("\n"));
        println!("Categories\n {}", doc.categories.join("\n"));
        //println!("Images {:?}", doc.images);
        //println!("Links {:?}", doc.links)
    }
}

fn add_heading(heading: &str, page_content: &mut String, doc: &mut Document) {
    let page: Page;

    doc.headings.push(heading.to_string());
    page_content.push_str(heading);
    page_content.push('\n');
}
