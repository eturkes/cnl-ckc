% drs_dump.pl — trusted DRS driver for `ckc certify` (contract m6, R41).
% Loads only upstream APE modules from a staged tree (never the fork
% ace_to_pl.pl), reads an optional ulex then the ACE text (strict UTF-8),
% parses with acetext_to_drs/8 (guessing + catching off, as the fork does),
% and prints three canonical lines under ONE numbervars pass:
%   sentences(Ss).  drs(Drs).  messages(Ms).
% Usage: swipl -q -f none -F none -s drs_dump.pl -g main -t 'halt(9)' -- <stage-tree> [<ulex>] < ace.txt
:- initialization(main, main).

main :-
    current_prolog_flag(argv, Argv),
    ( Argv = [Tree|Rest] -> true ; halt(2) ),
    atom_concat(Tree, '/prolog/parser/ace_to_drs.pl', Parser),
    use_module(Parser),
    atom_concat(Tree, '/prolog/lexicon/ulex.pl', UlexMod),
    use_module(UlexMod),
    ( Rest = [Ulex] ->
        ulex:discard_ulex,
        read_file_to_string(Ulex, UText, [encoding(utf8)]),
        setup_call_cleanup(open_string(UText, US), ulex:read_ulex(US), close(US))
    ; Rest = [] -> true
    ; halt(2)
    ),
    set_stream(user_input, encoding(utf8)),
    read_stream_to_codes(user_input, Codes),
    atom_codes(Text, Codes),
    ace_to_drs:acetext_to_drs(Text, off, off, Sentences, _Trees, Drs, Messages, _Time),
    copy_term(Sentences-Drs-Messages, S1-D1-M1),
    numbervars(S1-D1-M1, 0, _),
    Opts = [quoted(true), character_escapes(true), ignore_ops(true), numbervars(true)],
    write_term(sentences(S1), Opts), write('.'), nl,
    write_term(drs(D1), Opts), write('.'), nl,
    write_term(messages(M1), Opts), write('.'), nl,
    halt(0).
