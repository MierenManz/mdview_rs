use mdview_lexer::structures::Node;
use regex::Regex;

#[inline(always)]
pub fn serialize(node: Node, node_string: &str) -> String {
  let attributes = node.node_info.attributes.unwrap();
  let mut string = String::from(node_string);

  if attributes.image_or_link {
    let image_reg = Regex::new("!\\[(.*)\\]\\((.*)\\)").unwrap();
    let href_reg = Regex::new("\\[(.*)\\]\\((.*)\\)").unwrap();

    let first_pass = image_reg.replace_all(
      &string,
      "<a href=\"$2\"><img border=\"0\" src=\"$2\">$1</a>",
    );

    string =
      String::from(href_reg.replace_all(&first_pass, "<a href=\"$2\">$1</a>"));
  }

  if attributes.bold_or_italics {
    let bold_reg = Regex::new("\\*\\*(.*)\\*\\*").unwrap();
    let italic_reg = Regex::new("\\*(.*)\\*").unwrap();

    let first_pass = bold_reg.replace_all(&string, "<b>$1</b>");

    string = String::from(italic_reg.replace_all(&first_pass, "<i>$1</i>"));
  }

  if attributes.strike {
    let strike_reg = Regex::new("\\~\\~(.*)\\~\\~").unwrap();

    string = String::from(strike_reg.replace_all(&string, "<del>$1</del>"));
  }

  return string;
}
