use parse_wiki_text::Node;
use std::io::Read;
use std::str::FromStr;
use xml::reader::XmlEvent;
use xml::EventReader;

enum ElementType {
    None,
    Title,
    Id,
    Timestamp,
    Text,
}

pub struct Page {
    pub id: String,
    pub title: String,
    pub raw_content: String,
    revision_id: String,
    pub timestamp: String,
    pub meta: bool,
}

pub struct Document<'a> {
    pub page: &'a Page,
    pub contents: Vec<String>,
    pub categories: Vec<String>,
    pub headings: Vec<String>,
    pub images: Vec<Image>,
    pub links: Vec<Text>,
}

#[derive(Debug)]
pub enum ImageType {
    Image,
    File,
}

impl FromStr for ImageType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "File" => Ok(ImageType::File),
            "Image" => Ok(ImageType::Image),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Image {
    pub target: String,
    pub target_type: ImageType,
    pub text: Text,
}

#[derive(Debug)]
pub enum Text {
    LinkText { text: String, link: Link },
    Text { text: String },
}

impl Text {
    pub fn clone_text(&self) -> String {
        return match self {
            Text::LinkText { text, .. } => String::from(text),
            Text::Text { text, .. } => String::from(text),
        };
    }
}

#[derive(Debug)]
pub enum Link {
    ExternalLink { target: String },
    Link { target: String },
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
                    //println!("tag:{} ", name.local_name);
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
                    //println!("data:{} ", data.to_string);
                }
                Ok(XmlEvent::EndElement { name }) => match name.local_name.as_str() {
                    "page" => {
                        let title = self.title.take().unwrap();
                        let meta = title.starts_with("Wikipedia:");
                        let page = Page {
                            id: self.id.take().unwrap(),
                            title,
                            raw_content: self.content.take().unwrap(),
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
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
        None
    }
}

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
