%start instruction_list
%%

instruction_list
    : instruction { $1 }
    | instruction_list instruction 
    ;

instruction
    : "TYPE" argument_list { $1 }
    ;

argument_list
    : "(" argument_list ")" { $2 }
    | argument_list "COMMA" argument {}
    ;

argument -> Result<u64, ()>
    : "INT"
        {
            let v = $1.map_err(|_| ())?;
            parse_int($lexer.span_str(v.span()))
        }
    ;

%%
fn parse_instruction(input: &str, )

fn parse_int(input: &str) -> {
    match input.parse::<u64>() {
        Ok(val) => Ok(val),
        Err(e) => {
            eprintln!("{} cannot be represented as u64", input);
            Err(())
        }
    }
}
