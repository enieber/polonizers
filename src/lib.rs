#[derive(Debug, PartialEq)]
enum PolonizeItem {
    Empty,
    Express,
    Number,
}

pub fn polonise(token: String) -> PolonizeItem {
  if token.is_empty() {
    return PolonizeItem::Empty;
  }

  if is_express(&token) {
    return PolonizeItem::Express;
  }

  if is_number(&token) {
    return PolonizeItem::Number;
  }

  return PolonizeItem::Empty;
}

fn is_express(token: &str) -> bool {
    match token {
        "-" => true,
        "+" => true,
        "*" => true,
        "/" => true,
        _ => false,
    }
}

fn is_number(token: &str) -> bool {
    match token {
        "1" => true,
        "2" => true,
        "3" => true,
        "4" => true,
        "5" => true,
        "6" => true,
        "7" => true,
        "8" => true,
        "9" => true,
        "0" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_empty_token() {
        let result = polonise(String::from(""));
        assert_eq!(result, PolonizeItem::Empty);
    }

    #[test]
    fn it_save_expression() {
        let result = polonise(String::from("*"));
        assert_eq!(result, PolonizeItem::Express);
    }

    #[test]
    fn it_save_number() {
        let result = polonise(String::from("1"));
        assert_eq!(result, PolonizeItem::Number);
    }
}
