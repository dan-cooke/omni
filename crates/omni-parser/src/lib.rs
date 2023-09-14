use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub omni);

mod lexer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn simple_parsing() {}
}
