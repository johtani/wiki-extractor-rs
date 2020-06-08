use log::{debug, error};
use std::io::Read;
use xml::reader::XmlEvent;
use xml::EventReader;

enum ElementType {
    None,
    Title,
    Id,
    Timestamp,
    Text,
}

#[derive(Debug)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub raw_content: String,
    pub revision_id: String,
    pub timestamp: String,
    pub meta: bool,
}

pub struct WikiPageIterator<R: Read> {
    parser: EventReader<R>,
    id: Option<String>,
    title: Option<String>,
    content: Option<String>,
    revision_id: Option<String>,
    timestamp: Option<String>,
    in_revision: bool,
    tag: ElementType,
}

impl<R: Read> WikiPageIterator<R> {
    pub fn new(xml: R) -> Self {
        WikiPageIterator {
            parser: EventReader::new(xml),
            id: None,
            title: None,
            content: None,
            revision_id: None,
            timestamp: None,
            in_revision: false,
            tag: ElementType::None,
        }
    }
}

impl<R: Read> Iterator for WikiPageIterator<R> {
    type Item = Page;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.parser.next() {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    match name.local_name.as_str() {
                        "revision" => {
                            self.in_revision = true;
                        }
                        "title" => self.tag = ElementType::Title,
                        "id" => self.tag = ElementType::Id,
                        "text" => self.tag = ElementType::Text,
                        "timestamp" => self.tag = ElementType::Timestamp,
                        _ => self.tag = ElementType::None,
                    }
                    //debug!("tag:{} ", name.local_name);
                }
                Ok(XmlEvent::Characters(data)) => {
                    match self.tag {
                        ElementType::None => {}
                        ElementType::Title => self.title = Some(data.to_string()),
                        ElementType::Id => {
                            if self.in_revision {
                                self.revision_id = Some(data.to_string())
                            } else {
                                self.id = Some(data.to_string())
                            }
                        }
                        ElementType::Timestamp => self.timestamp = Some(data.to_string()),
                        ElementType::Text => self.content = Some(data.to_string()),
                    }
                    //debug!("data:{} ", data.to_string);
                }
                Ok(XmlEvent::EndElement { name }) => match name.local_name.as_str() {
                    "page" => {
                        let title = self.title.take().unwrap();
                        debug!("Title is [{}]", title.to_string());
                        let meta =
                            title.starts_with("Wikipedia:") || title.starts_with("MediaWiki:");
                        let page = Page {
                            id: self.id.take().unwrap(),
                            title,
                            raw_content: match self.content {
                                None => String::new(),
                                Some(_) => self.content.take().unwrap(),
                            },
                            revision_id: self.revision_id.take().unwrap(),
                            timestamp: self.timestamp.take().unwrap(),
                            meta,
                        };
                        return Some(page);
                    }
                    "revision" => {
                        self.in_revision = false;
                    }
                    _ => self.tag = ElementType::None,
                },
                Ok(XmlEvent::EndDocument { .. }) => {
                    break;
                }
                Err(e) => {
                    error!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
        None
    }
}
