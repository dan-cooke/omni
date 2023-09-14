use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("service")]
    KwService,
    #[token("resource")]
    KwResource,
    #[token("operation")]
    KwOperation,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
}
