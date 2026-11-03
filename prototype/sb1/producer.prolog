:- module(sb1_producer, [sb1_main/0, sb1_produce/3]).
:- use_module(library(assoc)).
:- use_module(library(crypto)).
:- use_module(library(filesex)).
:- use_module(library(lists)).
:- use_module(library(readutil)).
:- use_module(library(wfs)).

% Untrusted SB1 prototype; the checker, not this process, decides acceptance.
sb1_main :-
    catch((current_prolog_flag(argv, Args), sb1_dispatch(Args)), Error, sb1_failure(Error)),
    halt.

sb1_dispatch(['--root', Root, '--slice', Identity, Slice, Out]) :- !,
    sb1_produce_loaded(sb1_load_slice(Root, Slice, Clauses), Clauses, Identity, Out).
sb1_dispatch(Args) :-
    sb1_args(Args, Root, Composition, Out), sb1_produce(Root, Composition, Out).

sb1_args(['--root', Root, Composition, Out], Root, Composition, Out) :- !.
sb1_args(Args, _, _, _) :- throw(sb1_error(load, usage(Args))).

sb1_produce(Root, Composition, Out) :-
    sb1_produce_loaded(sb1_load(Root, Composition, Clauses, Identity), Clauses, Identity, Out).

:- meta_predicate sb1_produce_loaded(0, -, -, +).
sb1_produce_loaded(Loader, Clauses, Identity, Out) :-
    sb1_stage(load, Loader, Load),
    sb1_stage(eval, sb1_evaluate(Model), Eval),
    sb1_stage(certificate, sb1_certify(Clauses, Model, Claims), Certificate),
    sb1_stage(write, sb1_write_outputs(Out, Identity, Model, Claims), Write),
    sb1_stage(write, sb1_write_phases(Out,
        [load-Load, eval-Eval, certificate-Certificate, write-Write]), _).

:- meta_predicate sb1_stage(+, 0, -).
sb1_stage(Stage, Goal, timing(Wall, CPU)) :-
    get_time(W0), statistics(cputime, C0),
    catch((call(Goal) -> true ; throw(goal_failed)), Error,
        throw(sb1_error(Stage, Error))),
    statistics(cputime, C1), get_time(W1), Wall is W1 - W0, CPU is C1 - C0.

sb1_failure(Error) :-
    ( Error = sb1_error(Stage0, Why) -> true ; Stage0 = load, Why = Error ),
    ( sb1_resource(Why) -> Stage = resource ; Stage = Stage0 ),
    format(user_error, 'sb1-producer ~w ', [Stage]),
    with_output_to(user_error, sb1_write_term(Why)), nl(user_error), halt(1).
sb1_resource(error(resource_error(_), _)).
sb1_resource(time_limit_exceeded).

sb1_indicator(guideline_schema_version/1).
sb1_indicator(guideline_document/4).
sb1_indicator(guideline_entity/4).
sb1_indicator(guideline_cardinality/5).
sb1_indicator(guideline_event/3).
sb1_indicator(guideline_arg/4).
sb1_indicator(guideline_pp/4).
sb1_indicator(guideline_property/4).
sb1_indicator(guideline_operator/3).
sb1_indicator(guideline_interval/6).
sb1_indicator(guideline_recurrence/4).
sb1_indicator(guideline_frequency/5).
sb1_indicator(guideline_order/4).
sb1_indicator(guideline_recurrence_window/7).
sb1_indicator(guideline_range/3).
sb1_indicator(guideline_grade/5).
sb1_indicator(guideline_preference/4).
sb1_indicator(guideline_qualifier/4).
sb1_indicator(guideline_unit/3).
sb1_indicator(guideline_exclusion/1).
sb1_indicator(guideline_supersession/4).
sb1_indicator(guideline_evaluation_date/1).

sb1_literal(G) :- nonvar(G), functor(G, Name, Arity), sb1_indicator(Name/Arity).
sb1_builtin(G) :- nonvar(G), functor(G, Name, Arity), sb1_builtin_indicator(Name, Arity).
sb1_builtin_indicator(guideline_satisfies, 4).
sb1_builtin_indicator(guideline_satisfies, 6).
sb1_builtin_indicator(guideline_product, 6).
sb1_builtin_indicator(guideline_quotient, 6).

sb1_load(Root, Composition, Clauses, Identity) :-
    ( current_prolog_flag(version_data, swi(10, 0, 2, _)) -> true
    ; throw(expected_swipl('10.0.2')) ),
    sb1_reset,
    sb1_path(Root, 'guidelines/schema.pl', Schema),
    sb1_path(Root, Composition, CompositionPath),
    read_file_to_string(CompositionPath, Text, [encoding(utf8)]),
    split_string(Text, "\n", "\r", Lines),
    findall(file(Listed, Path),
        ( member(Line, Lines), Line \== "", \+ sub_string(Line, 0, 1, _, "#"),
          atom_string(Listed, Line), sb1_path(Root, Listed, Path) ), Files),
    sb1_identity(Schema, Files, Identity),
    load_files(sb1_world:Schema, [silent(true), if(true)]),
    sb1_read_files(Files, 1, Clauses),
    forall(member(clause(_, H, B, _), Clauses), sb1_assert(sb1_world, H, B)),
    ( setof(V, sb1_world:guideline_schema_version(V), [2]) -> true
    ; throw(schema_identifier(expected(2))) ).

% The orchestrator binds identity + original ids; this process reads no unsliced
% documents. An empty slice is a complete empty program, not a missing output.
sb1_load_slice(Root, Slice, Clauses) :-
    ( current_prolog_flag(version_data, swi(10, 0, 2, _)) -> true
    ; throw(expected_swipl('10.0.2')) ),
    sb1_reset,
    sb1_path(Root, 'guidelines/schema.pl', Schema),
    load_files(sb1_world:Schema, [silent(true), if(true)]),
    sb1_path(Root, Slice, Path),
    setup_call_cleanup(open(Path, read, Stream, [encoding(utf8)]),
        sb1_read_slice(Stream, Clauses), close(Stream)),
    findall(Id, member(clause(Id, _, _, _), Clauses), Ids), sort(Ids, Unique),
    ( same_length(Ids, Unique) -> true ; throw(duplicate_slice_clause_id) ),
    forall(member(clause(_, H, B, _), Clauses), sb1_assert(sb1_world, H, B)).
sb1_read_slice(Stream, Clauses) :-
    read_term(Stream, Term, [syntax_errors(error)]),
    ( Term == end_of_file -> Clauses = []
    ; nonvar(Term), Term = sb1_clause(Doc, Ordinal, Code),
      integer(Doc), Doc >= 1, integer(Ordinal), Ordinal >= 1, nonvar(Code)
    -> ( Code = (H :- B) -> true ; H = Code, B = true ),
       ( sb1_literal(H) -> true ; throw(unsupported_head(Doc, Ordinal, H)) ),
       sb1_nonlocal(H, B, Variables), sb1_validate_body(B),
       sb1_safe(B, Variables, [], Bound), term_variables(H, HeadVars), sb1_bound(HeadVars, Bound),
       ( B == true, \+ ground(H) -> throw(nonground_fact(Doc, Ordinal)) ; true ),
       Clauses = [clause(Doc-Ordinal, H, B, Variables)|Rest], sb1_read_slice(Stream, Rest)
    ; throw(malformed_slice_clause(Term)) ).

sb1_path(Root, Listed, Path) :-
    ( is_absolute_file_name(Listed) -> Path = Listed ; directory_file_path(Root, Listed, Path) ).

sb1_reset :-
    abolish_all_tables,
    retractall(sb1_ascii_atom(_)), retractall(sb1_unicode_atom(_)),
    forall(sb1_indicator(Name/Arity),
        ( functor(G, Name, Arity),
          dynamic(sb1_world:Name/Arity), retractall(sb1_world:G), table(sb1_world:Name/Arity),
          dynamic(sb1_model:Name/Arity), retractall(sb1_model:G) )),
    forall((member(Module, [sb1_world, sb1_model]), sb1_keyed(Name, Arity, Fast)),
        sb1_keyed_reset(Module, Name, Arity, Fast)).

% SWI's role/count indexes scan every shard. A flat identity prefix routes
% these joins; the original arguments still decide every match, Deps included.
sb1_keyed(guideline_arg, 4, guideline_arg_keyed).
sb1_keyed(guideline_cardinality, 5, guideline_cardinality_keyed).
sb1_keyed_reset(Module, Name, Arity, Fast) :-
    FastArity is Arity + 1, functor(Stored, Fast, FastArity),
    dynamic(Module:Fast/FastArity), retractall(Module:Stored),
    functor(H, Name, Arity), arg(2, H, Id), H =.. [Name|Args],
    Target =.. [Fast, Key|Args],
    assertz(Module:(H :- sb1_producer:sb1_key(Id, Key), Target)).
sb1_assert(Module, H, B) :-
    functor(H, Name, Arity),
    ( sb1_keyed(Name, Arity, Fast)
    -> arg(2, H, Id), sb1_key(Id, Key), H =.. [Name|Args],
       Stored =.. [Fast, Key|Args], assertz(Module:(Stored :- B))
    ; assertz(Module:(H :- B)) ).
sb1_key(Id, Key) :-
    ( nonvar(Id), Id = '$guideline_id'(Kind, Doc, S, Slot, _), ground(Kind-Doc-S-Slot)
    -> term_hash(k(Kind, Doc, S, Slot), Key)
    ; atomic(Id) -> Key = Id
    ; true ).

sb1_identity(Schema, Files, Identity) :-
    sb1_file_digest(Schema, Size, Hash),
    format(string(First), 'schema\u0000~d\u0000~w\n', [Size, Hash]),
    crypto_context_new(C0, [algorithm(sha256), encoding(utf8)]),
    crypto_data_context(First, C0, C1),
    sb1_identity_files(Files, C1, C), crypto_context_hash(C, Identity).
sb1_identity_files([], C, C).
sb1_identity_files([file(Listed, Path)|Files], C0, C) :-
    sb1_file_digest(Path, Size, Hash),
    format(string(Record), 'file\u0000~w\u0000~d\u0000~w\n', [Listed, Size, Hash]),
    crypto_data_context(Record, C0, C1), sb1_identity_files(Files, C1, C).
sb1_file_digest(Path, Size, Hash) :-
    size_file(Path, Size), crypto_file_hash(Path, Hash, [algorithm(sha256), encoding(octet)]).

sb1_read_files([], _, []).
sb1_read_files([file(_, Path)|Files], Doc, Clauses) :-
    setup_call_cleanup(open(Path, read, S, [encoding(utf8)]),
        sb1_read_clauses(S, Doc, 1, Clauses, Rest), close(S)),
    Next is Doc + 1, sb1_read_files(Files, Next, Rest).
sb1_read_clauses(S, Doc, Ordinal, Clauses, Rest) :-
    read_term(S, Term, [syntax_errors(error)]),
    ( Term == end_of_file -> Clauses = Rest
    ; var(Term) -> throw(variable_clause(Doc, Ordinal))
    ; Term = (:- Directive)
    -> ( ground(Directive), sb1_declaration(Directive) -> true ; throw(unsupported_directive(Directive)) ),
       sb1_read_clauses(S, Doc, Ordinal, Clauses, Rest)
    ; ( Term = (H :- B) -> true ; H = Term, B = true ),
      ( sb1_literal(H) -> true ; throw(unsupported_head(Doc, Ordinal, H)) ),
      sb1_nonlocal(H, B, Variables),
      sb1_validate_body(B), sb1_safe(B, Variables, [], Bound),
      term_variables(H, HeadVars), sb1_bound(HeadVars, Bound),
      ( B == true, \+ ground(H) -> throw(nonground_fact(Doc, Ordinal)) ; true ),
      Clauses = [clause(Doc-Ordinal, H, B, Variables)|Next],
      N is Ordinal + 1, sb1_read_clauses(S, Doc, N, Next, Rest) ).

sb1_declaration(multifile(I)) :- sb1_indicator(I).
sb1_declaration(discontiguous(I)) :- sb1_indicator(I).
sb1_validate_body(G) :-
    ( var(G) -> throw(variable_goal)
    ; G == true -> true
    ; G = (A, B) -> sb1_validate_body(A), sb1_validate_body(B)
    ; G = (\+ Inner) -> sb1_validate_negative(Inner)
    ; sb1_literal(G) -> true
    ; sb1_builtin(G) -> true
    ; throw(unsupported_goal(G)) ).
sb1_validate_negative(G) :-
    ( var(G) -> throw(variable_negative_goal)
    ; G = (A, B) -> sb1_validate_negative(A), sb1_validate_negative(B)
    ; sb1_literal(G) -> true
    ; sb1_builtin(G) -> true
    ; throw(unsupported_negative_goal(G)) ).

sb1_nonlocal(H, B, Variables) :-
    term_variables(H-B, All), sb1_outside(B, Positive, Negations),
    term_variables(H-Positive, Outside),
    include(sb1_nonlocal_var(Outside, Negations), All, Variables).
sb1_outside(G, Positive, Negations) :-
    ( nonvar(G), G = (A, B)
    -> sb1_outside(A, X, N1), sb1_outside(B, Y, N2), Positive = [X, Y], append(N1, N2, Negations)
    ; nonvar(G), G = (\+ Inner)
    -> Positive = [], term_variables(Inner, Vs), Negations = [Vs]
    ; Positive = G, Negations = [] ).
sb1_nonlocal_var(Outside, Negations, V) :-
    ( sb1_same_member(V, Outside) -> true
    ; include(sb1_same_member(V), Negations, Occurrences), Occurrences = [_, _|_] ).
sb1_same_member(V, [W|_]) :- V == W, !.
sb1_same_member(V, [_|Rest]) :- sb1_same_member(V, Rest).
sb1_bound(Vars, Bound) :-
    ( forall(member(V, Vars), sb1_same_member(V, Bound)) -> true ; throw(unsafe_variables) ).
sb1_safe(G, Nonlocal, Bound0, Bound) :-
    ( G == true -> Bound = Bound0
    ; G = (A, B) -> sb1_safe(A, Nonlocal, Bound0, Bound1), sb1_safe(B, Nonlocal, Bound1, Bound)
    ; G = (\+ Inner)
    -> term_variables(Inner, Vars), include(sb1_same_member_in(Nonlocal), Vars, Outer),
       sb1_bound(Outer, Bound0), sb1_safe(Inner, Nonlocal, Bound0, _), Bound = Bound0
    ; sb1_builtin(G)
    -> sb1_inputs(G, Inputs), term_variables(Inputs, Vars), sb1_bound(Vars, Bound0),
       term_variables(G-Bound0, Bound)
    ; term_variables(G-Bound0, Bound) ).
sb1_same_member_in(Vars, V) :- sb1_same_member(V, Vars).
sb1_inputs(G, Inputs) :-
    ( G = guideline_product(A, B, C, D, _, _) -> Inputs = [A, B, C, D]
    ; G = guideline_quotient(A, B, C, D, _, _) -> Inputs = [A, B, C, D]
    ; Inputs = G ).

sb1_evaluate(Model) :-
    findall(G,
        ( sb1_indicator(Name/Arity), functor(G, Name, Arity),
          call_delays(sb1_world:G, Delays),
          ( Delays == true -> true ; throw(undefined_answer(G, Delays)) ),
          ( ground(G) -> true ; throw(nonground_answer(G)) ) ), Facts),
    sort(Facts, Model).

sb1_certify(Clauses, Model, Claims) :-
    forall(member(Fact, Model), sb1_assert(sb1_model, Fact, true)),
    maplist(sb1_node, Model, Pairs), list_to_assoc(Pairs, Nodes),
    % Backtracking would undo nb_linkarg bindings. Each clause's ground batch
    % is traversed deterministically; no composition-wide instance copy stays live.
    State = agenda([], [], -1),
    include(sb1_fact_clause, Clauses, Facts), maplist(sb1_seed(Nodes, State), Facts),
    maplist(sb1_clause_instances(Nodes, State), Clauses),
    sb1_agenda(State), maplist(sb1_claim(Nodes), Model, Claims).
sb1_clause_instances(Nodes, State, clause(Id, H, B, Vars)) :-
    ( B == true -> true
    ; findall(instance(H, support(Id, Valuation), Unique),
          ( sb1_body(B, Positive), sb1_valuation(Vars, 1, Valuation),
            ( ground(H-Valuation-Positive) -> true ; throw(nonground_instance(Id)) ),
            sort(Positive, Unique) ), Instances),
      maplist(sb1_register_instance(Nodes, State), Instances) ).
sb1_fact_clause(clause(_, _, B, _)) :- B == true.
sb1_seed(Nodes, State, clause(Id, H, true, [])) :-
    sb1_ready(Nodes, State, H, support(Id, []), 0).
sb1_register_instance(Nodes, State, instance(H, Support, Positive)) :-
    sb1_register(Positive, Nodes, State, H, Support).
sb1_node(Fact, Fact-node(unset, none, [])).
sb1_body(true, []) :- !.
sb1_body((A, B), Positive) :- !,
    sb1_body(A, X), sb1_body(B, Y), append(X, Y, Positive).
sb1_body(\+ G, []) :- !, \+ sb1_truth(G).
sb1_body(G, []) :- sb1_builtin(G), !, call(sb1_world:G).
sb1_body(G, [G]) :- call(sb1_model:G).
sb1_truth((A, B)) :- !, sb1_truth(A), sb1_truth(B).
sb1_truth(G) :- sb1_builtin(G), !, call(sb1_world:G).
sb1_truth(G) :- call(sb1_model:G).
sb1_valuation([], _, []).
sb1_valuation([Value|Vars], N, [=(Key, Value)|Values]) :-
    atom_concat(v, N, Key), Next is N + 1, sb1_valuation(Vars, Next, Values).

% Each positive fact owns its waiting incidences. Seeds precede level-1
% instances; settling level L adds only L+1, so FIFO preserves the heap's
% lowest-level/insertion-order choice. Duplicate heads release nothing twice.
sb1_register(Positive, Nodes, State, H, Support) :-
    sb1_lookup(H, Nodes, HeadNode), length(Positive, Count),
    Pending = pending(HeadNode, Support, Count, 0),
    ( Count =:= 0 -> sb1_enqueue(State, 1, HeadNode, Support)
    ; sb1_waiters(Positive, Nodes, Pending) ).
sb1_waiters([], _, _).
sb1_waiters([Fact|Facts], Nodes, Pending) :-
    sb1_lookup(Fact, Nodes, Node), arg(3, Node, Waiting),
    nb_linkarg(3, Node, [Pending|Waiting]), sb1_waiters(Facts, Nodes, Pending).
sb1_lookup(Fact, Nodes, Node) :-
    ( get_assoc(Fact, Nodes, Node) -> true ; throw(model_not_closed(Fact)) ).
sb1_ready(Nodes, State, H, Support, Level) :-
    sb1_lookup(H, Nodes, Node), sb1_enqueue(State, Level, Node, Support).
sb1_enqueue(State, Level, Node, Support) :-
    arg(2, State, Back), nb_linkarg(2, State, [ready(Level, Node, Support)|Back]).
sb1_agenda(State) :-
    arg(1, State, Front0),
    ( Front0 == []
    -> arg(2, State, Back), reverse(Back, Front),
       nb_linkarg(1, State, Front), nb_setarg(2, State, [])
    ; Front = Front0 ),
    ( Front = [ready(Level, Node, Support)|Rest]
    -> nb_linkarg(1, State, Rest), arg(3, State, Previous),
       ( Level >= Previous -> true ; throw(fifo_level_order(Previous, Level)) ),
       nb_setarg(3, State, Level),
       ( arg(1, Node, unset)
       -> nb_setarg(1, Node, Level), nb_setarg(2, Node, Support),
          arg(3, Node, Waiting),
          maplist(sb1_release_at(State, Level), Waiting), nb_setarg(3, Node, [])
       ; true ),
       sb1_agenda(State)
    ; true ).
sb1_release_at(State, Level, Pending) :- sb1_release(Pending, State, Level).
sb1_release(Pending, State, Level) :-
    Pending = pending(Node, Support, Count0, Max0),
    Count is Count0 - 1, Max is max(Max0, Level),
    nb_setarg(3, Pending, Count), nb_setarg(4, Pending, Max),
    ( Count =:= 0 -> Next is Max + 1, sb1_enqueue(State, Next, Node, Support) ; true ).
sb1_claim(Nodes, Fact, claim(Fact, Id, Valuation, Level)) :-
    get_assoc(Fact, Nodes, node(Level, Support, _)),
    ( Level == unset -> throw(unfounded_residual(Fact))
    ; Support = support(Id, Valuation) ).

sb1_write_outputs(Out, Identity, Model, Claims) :-
    make_directory_path(Out),
    forall(member(Name, ['model.txt', 'certificate.txt', 'producer-phases.tsv']),
        ( directory_file_path(Out, Name, Path),
          ( exists_file(Path) ; exists_directory(Path) ; catch(read_link(Path, _, _), _, fail) )
          -> throw(output_exists(Path)) ; true )),
    sb1_output(Out, 'model.txt', sb1_write_model(Model)),
    sb1_output(Out, 'certificate.txt', sb1_write_certificate(Identity, Claims)).
:- meta_predicate sb1_output(+, +, 0).
sb1_output(Out, Name, Goal) :-
    directory_file_path(Out, Name, Path),
    setup_call_cleanup(open(Path, write, Stream, [encoding(utf8), newline(posix)]),
        with_output_to(Stream, Goal), close(Stream)).
sb1_write_model(Model) :-
    forall(member(Fact, Model), (sb1_write_term(Fact), write('.'), nl)).
sb1_write_certificate(Identity, Claims) :-
    format('sb1-certificate 1~ncomposition ~w~n', [Identity]),
    sb1_write_claims(Claims, 1).
sb1_write_claims([], _).
sb1_write_claims([claim(_, Doc-Ordinal, Valuation, Level)|Claims], Index) :-
    format('~d\t~d:~d\t', [Index, Doc, Ordinal]),
    sb1_write_term(Valuation), format('\t~d~n', [Level]),
    Next is Index + 1, sb1_write_claims(Claims, Next).
sb1_write_phases(Out, Phases) :-
    sb1_output(Out, 'producer-phases.tsv', sb1_phase_rows(Phases)).
sb1_phase_rows(Phases) :-
    format('phase\tseconds\tcpu~n'),
    forall(member(Name-timing(Wall, CPU), Phases), format('~w\t~9f\t~9f~n', [Name, Wall, CPU])).

:- dynamic sb1_ascii_atom/1, sb1_unicode_atom/1.
sb1_write_term(Term) :-
    ( sb1_term_unicode(Term)
    -> write_term(Term, [quoted(true), numbervars(true), character_escapes(true),
           ignore_ops(true), portray_goal(sb1_producer:sb1_portray)])
    ; write_term(Term, [quoted(true), numbervars(true), character_escapes(true), ignore_ops(true)]) ).
sb1_term_unicode(Term) :-
    sub_term(Atom, Term), atom(Atom), sb1_atom_unicode(Atom), !.
sb1_atom_unicode(Atom) :-
    ( sb1_ascii_atom(Atom) -> fail
    ; sb1_unicode_atom(Atom) -> true
    ; atom_codes(Atom, Codes),
      ( member(C, Codes), C >= 128
      -> assertz(sb1_unicode_atom(Atom))
      ; assertz(sb1_ascii_atom(Atom)), fail ) ).
% v1 quotes non-ASCII atoms and keeps their Unicode scalars literal (R22).
sb1_portray(Atom, _) :-
    atom(Atom), atom_codes(Atom, Codes), member(C, Codes), C >= 128, !,
    put_code(39), maplist(sb1_atom_code, Codes), put_code(39).
sb1_atom_code(92) :- !, write('\\\\').
sb1_atom_code(39) :- !, write('\\\'').
sb1_atom_code(C) :- nth0(I, [7, 8, 9, 10, 11, 12, 13], C), !,
    nth0(I, [a, b, t, n, v, f, r], E), put_code(92), write(E).
sb1_atom_code(C) :- ( C < 32 ; C =:= 127 ), !, format('\\u~|~`0t~16R~4+', [C]).
sb1_atom_code(C) :- put_code(C).
