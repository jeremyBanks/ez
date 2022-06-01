// use crate::*;

// pub struct DoopItemBinding {
//     lhs: Tokens,
//     rhs: Tokens,
// }

// pub enum DoopItem {
//     TokensAssignment(DoopItemBinding),
//     ListAssignment(DoopItemBinding),
//     GeneratingBlock(Vec<DoopItemBinding>, Tokens),
// }

// pub fn parse(input: &Tokens) -> Result<Vec<DoopItem>, Tokens> {
//     let mut items = Vec::new();

//     for line in input.split_lines() {
//         if line.is_empty() || line.punct().map(Punct::as_char) == Some(';') {
//             continue;
//         }

//         let mut items = vec![];
//         enum Item {
//             TokensAssignment { lhs: Ident, rhs: Tokens },
//             TokensListAssignment { lhs: Ident, rhs: Tokens },
//             EmitForLoop { body: Tokens, bindings: Vec<(Tokens, Tokens)> },
//         }

//         if let Some(braced) = line.braced() {
//             items.push(Item::EmitForLoop {
//                 body: braced.into_tokens(),
//                 bindings: vec![(Tokens::from("()"), Tokens::from("()"))],
//             });
//         } else if let Some(TokenTree::Ident(ident)) = line.first() {
//             let keyword = ident.to_string();

//             if keyword == "for" {
//                 let bindings = Vec::new();
//                 let mut iter = line.iter().peekable();
//                 let body: Tokens;
//                 loop {
//                     match iter.peek() {
//                         Some(TokenTree::Ident(ident)) if ident.to_string() ==
// "for" => {                             iter.next();
//                         }
//                         Some(tree) => {
//                             return TokenTree::clone(tree).error("expected
// `for` keyword");                         }
//                         None => {
//                             return line.clone().error("expected `for`
// keyword, got end of line");                         }
//                     }

//                     let target = match iter.next() {
//                         Some(tt @ TokenTree::Ident(id)) =>
// tt.clone().into_tokens(),                         Some(tt @
// TokenTree::Group(g))                             if g.delimiter() ==
// Delimiter::Parenthesis =>
// tt.clone().into_tokens(),                         Some(tree) => {
//                             return tree
//                                 .clone()
//                                 .error("expected identifier or tuple binding
// target");                         }
//                         None =>
//                             return line.clone().error(
//                                 "expected identifier or tuple binding target,
// got end of line",                             ),
//                     };

//                     match iter.next() {
//                         Some(TokenTree::Ident(ident)) if ident.to_string() ==
// "in" => (),                         Some(tree) => {
//                             return tree.clone().error("expected `in`
// keyword");                         }
//                         None =>
//                             return line.clone().error("expected `in` keyword,
// got end of line"),                     };

//                     let mut token_list = Tokens::new();

//                     loop {
//                         match iter.peek() {
//                             Some(TokenTree::Ident(ident)) if
// ident.to_string() == "for" => {                                 continue;
//                             }
//                             Some(TokenTree::Group(g)) if g.delimiter() ==
// Delimiter::Brace => {
// items.push(Item::EmitForLoop {                                     body:
// line.clone().into_tokens(),                                     bindings:
// bindings.clone(),                                 });
//                                 break;
//                             }
//                             Some(tt) => {
//
// token_list.extend(TokenTree::clone(tt).into_tokens());
// }                             None => {
//                                 return line.clone().error(
//                                     "expected `for` keyword or braced group,
// got end of line",                                 );
//                             }
//                         }
//                     }
//                 }
//             } else if matches!(keyword.as_ref(), "let" | "static" | "type" |
// "const") {                 return line.error("TODO: implement parsing
// assignment");             } else {
//                 return line.error(format!("unexpected keyword: {keyword}"));
//             }
//         } else {
//             return line.error("expected keyword or braced block");
//         }
//     }

//     Ok(items)
// }
