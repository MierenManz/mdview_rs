use mdview_lexer::structures::Node;
use mdview_lexer::structures::TextAttributes;
use regex::Regex;

// This function is susceptible to injection attacks.
// The user input needs to be HTML escaped, or this should be documented.
pub(crate) fn serialize(node: Node, node_string: &str) -> String {
    let attributes = match node.info.attributes {
        Some(attr) => attr,
        None => TextAttributes {
            inline_code: false,
            image_or_link: false,
            bold_or_italics: false,
            strike: false,
        },
    };

    let mut string = String::from(node_string);

    if attributes.image_or_link {
        let image_reg = Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
        let href_reg = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();

        let first_pass = image_reg.replace_all(
            &string,
            "<a href=\"$2\"><img border=\"0\" src=\"$2\">$1</a>",
        );

        string = String::from(
            href_reg.replace_all(&first_pass, "<a href=\"$2\">$1</a>"),
        );
    }

    if attributes.bold_or_italics {
        let bold_reg = Regex::new(r"\*\*(.*?)\*\*").unwrap();
        let italic_reg = Regex::new(r"\*(.*?)\*").unwrap();

        let first_pass = bold_reg.replace_all(&string, "<b>$1</b>");

        string = String::from(italic_reg.replace_all(&first_pass, "<i>$1</i>"));
    }

    if attributes.strike {
        let strike_reg = Regex::new(r"\~\~(.*?)\~\~").unwrap();

        string = String::from(strike_reg.replace_all(&string, "<del>$1</del>"));
    }

    if attributes.inline_code {
        let inline_reg = Regex::new("`(.*?)`").unwrap();

        string =
            String::from(inline_reg.replace_all(&string, "<code>$1</code>"));
    }

    string
}
