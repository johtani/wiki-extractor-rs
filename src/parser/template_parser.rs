use crate::parser::common_parser::parse_text_only;
use log::{debug, trace};
use parse_wiki_text::{Node, Parameter};

pub enum TemplateType {
    Unknown,
    Lang,
    Redirect,
    Otheruses,
    TemporaryLink,
    Unicode,
}

pub fn parse_template_type(nodes: &Vec<Node>) -> TemplateType {
    let name = parse_text_only(nodes);
    let template_type = match name.as_str() {
        "lang" => TemplateType::Lang,
        "lang-en" => TemplateType::Lang,
        "Redirect" => TemplateType::Redirect,
        "Otheruses" => TemplateType::Otheruses,
        "仮リンク" => TemplateType::TemporaryLink,
        "unicode" => TemplateType::Unicode,
        "IPA" => TemplateType::Unicode,
        _ => {
            debug!("[WARN] Unknown Template Type... [{}]", name);
            TemplateType::Unknown
        }
    };
    return template_type;
}

pub fn parse_template(name: &Vec<Node>, parameters: &Vec<Parameter>) -> Option<String> {
    return match parse_template_type(name) {
        TemplateType::Lang => Some(get_lang_template_text(&parameters)),
        TemplateType::TemporaryLink => Some(get_temporary_link_template_text(&parameters)),
        TemplateType::Unicode => Some(get_temporary_link_template_text(&parameters)),
        TemplateType::Otheruses => None,
        TemplateType::Redirect => None,
        TemplateType::Unknown => {
            trace!("Params: [{:?}]", parameters);
            None
        }
    };
}

pub fn get_lang_template_text(nodes: &Vec<Parameter>) -> String {
    let len = nodes.len();
    let str = if len >= 2 {
        parse_text_only(&nodes.get(1).unwrap().value)
    } else if len == 1 {
        parse_text_only(&nodes.get(0).unwrap().value)
    } else {
        debug!("[WARN]Template param is 0. Unexpected value... {:?}", nodes);
        String::new()
    };
    return str;
}

pub fn get_temporary_link_template_text(nodes: &Vec<Parameter>) -> String {
    let str = if nodes.len() > 0 {
        parse_text_only(&nodes.get(0).unwrap().value)
    } else {
        debug!("[WARN]Template param is 0. Unexpected value... {:?}", nodes);
        String::new()
    };
    return str;
}
