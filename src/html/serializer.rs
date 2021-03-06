use lazy_static::lazy_static;
use mdview_lexer::structures::Node;
use regex::Regex;

// This function is susceptible to injection attacks.
// The user input needs to be HTML escaped, or this should be documented.
pub(crate) fn serialize(node: Node, node_string: &str) -> String {
    let mut string = String::from(node_string);

    let attributes = match node.info.attributes {
        Some(attr) => attr,
        None => return string,
    };

    if attributes.image_or_link {
        lazy_static! {
            static ref IMAGE_REG: Regex =
                Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
            static ref HREF_REG: Regex =
                Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
        }

        let first_pass = IMAGE_REG.replace_all(
            &string,
            "<a href=\"$2\"><img border=\"0\" src=\"$2\">$1</a>",
        );

        string = String::from(
            HREF_REG.replace_all(&first_pass, "<a href=\"$2\">$1</a>"),
        );
    }

    if attributes.bold_or_italics {
        string = format!(
            "<i>{}</i>",
            format!("<b>{}</b>", string.replace("**", "")).replace("*", "")
        );
    }

    if attributes.strike {
        string = format!("<del>{}</del>", string.replace("~~", ""));
    }

    if attributes.inline_code {
        lazy_static! {
            static ref INLINE_REG: Regex = Regex::new("`(.*?)`").unwrap();
        }
        string =
            String::from(INLINE_REG.replace_all(&string, "<code>$1</code>"));
    }

    string
}
