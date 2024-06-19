use cfgrammar::yacc::YaccKind;
use lrlex::CTLexerBuilder;

fn main() {
    CTLexerBuilder::new()
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Original(
                cfgrammar::yacc::YaccOriginalActionKind::GenericParseTree,
            ))
            .grammar_in_src_dir("grammar.y")
            .unwrap()
        })
        .lexer_in_src_dir("grammar.l")
        .unwrap()
        .build()
        .unwrap();
}
