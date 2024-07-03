%start instruction_list
%%

instruction_list
    : instruction
    | instruction_list instruction
    ;

instruction
    : "TYPE" argument_list
    ;

argument_list
    : "INT"
    | "INT" "COMMA" argument_list
    ;

%%
