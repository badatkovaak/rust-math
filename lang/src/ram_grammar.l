%%
(0|[1-9][0-9]*)   "INT"
(?i)(succ|s|zero|z|jump|j|copy|c) "TYPE"
\(                "("
\)                ")"
,                 "COMMA"
[ \t\n]+          ;
