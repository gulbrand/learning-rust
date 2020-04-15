use std::env;

#[derive(Clone)]
struct CommentBlock {
    text: String,
    width: i16,
    min_height: i16,
    min_width_pad: i16,
}
#[allow(dead_code)]
fn generate_comment_block(comment_block: CommentBlock) -> String {
    let result: String = comment_block.text;

    let header =
        format!("/{}",
                (0..comment_block.width - 1)
                    .map(|_| "*")
                    .collect::<String>());

    let footer =
        format!(" {}/",
                (0..comment_block.width - 2)
                    .map(|_| "*")
                    .collect::<String>());

    format!("{}\n{}\n{}", header, result, footer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn simple_test() {
        let comment_block =
            CommentBlock {
                text: "This is a comment".to_string(),
                width: 50,
                min_height: 3,
                min_width_pad: 3,
            };

        let comment_block_string =
            generate_comment_block(comment_block);

        println!("{}", comment_block_string);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("what do you want to write in the block?");
    }
}