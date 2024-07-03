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
    : argument
    | argument_list "COMMA" argument
    ;

argument
    : "INT"

%%
