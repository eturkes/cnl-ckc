:- module(explanation, [assemble_result_terms/7, validate_answer_terms/1]).

:- set_prolog_flag(encoding, utf8).

:- use_module(library(lists), [append/3]).
:- use_module(drs_canon, [canonical_line/2]).

/*
Witness expansion and proof-certificate replay. Construction follows the first
witness retained by the kernel. Replay independently matches every cited clause
with an explicit var(N) -> named(Atom) map and checks each NAF leaf against the
completed kernel store before any result term is returned. A replay failure is
an internal invariant exception, never an input rejection.
*/
assemble_result_terms(Document, ProgramDigest, GoalId, GoalAtom, Outcome,
        Store, Terms) :-
    ( Outcome == proved ->
        build_certificate(GoalAtom, Store, Proof),
        replay_certificate(GoalAtom, Proof, Store),
        Terms = [ cnl_answer_record(2),
                  Document,
                  program(sha256(ProgramDigest)),
                  answer(GoalId, GoalAtom, proved),
                  Proof
                ]
    ; Outcome == not_proved ->
        ( store_witness(GoalAtom, Store, _) ->
            invariant(not_proved_goal_has_witness)
        ; true
        ),
        Terms = [ cnl_answer_record(2),
                  Document,
                  program(sha256(ProgramDigest)),
                  answer(GoalId, GoalAtom, not_proved)
                ]
    ; has_functor(Outcome, answers, 1) ->
        arg(1, Outcome, Answers),
        build_wh_certificates(Answers, Store, Proofs),
        Prefix = [ cnl_answer_record(2),
                   Document,
                   program(sha256(ProgramDigest)),
                   answer(GoalId, wh(who), GoalAtom, answers(Answers))
                 ],
        append(Prefix, Proofs, Terms)
    ; invariant(outcome(Outcome))
    ).

build_wh_certificates(Answers, _, []) :-
    Answers == [],
    !.
build_wh_certificates(Answers, Store, [Proof|Proofs]) :-
    has_functor(Answers, '[|]', 2),
    !,
    arg(1, Answers, Atom),
    arg(2, Answers, Rest),
    build_certificate(Atom, Store, Proof),
    replay_certificate(Atom, Proof, Store),
    build_wh_certificates(Rest, Store, Proofs).
build_wh_certificates(_, _, _) :-
    invariant(wh_answers_shape).

/*
The run CLI calls this only on reparsed generated terms. Every answer record
first passes the shared v2 envelope/layout gate; the payload then selects the
yes/no or wh grammar and exact proof-correspondence gate.
*/
validate_answer_terms(Terms) :-
    ( answer_record_layout(
          Terms, Header, Document, Program, Answer, Proofs) ->
        validate_answer_common_layout(Header, Document, Program),
        validate_answer_payload(
            Header, Document, Program, Answer, Proofs)
    ; invariant(generated_answer_layout)
    ).

validate_answer_common_layout(Header, Document, Program) :-
    ( Header == cnl_answer_record(2),
      has_functor(Document, document, 3),
      generated_program_digest(Program) ->
        true
    ; invariant(generated_answer_envelope)
    ).

validate_answer_payload(Header, Document, Program, Answer, Proofs) :-
    ( has_functor(Answer, answer, 4) ->
        validate_wh_answer_record(
            Header, Document, Program, Answer, Proofs)
    ; has_functor(Answer, answer, 3) ->
        validate_yes_no_answer_record(Answer, Proofs)
    ; invariant(generated_answer_shape)
    ).

validate_yes_no_answer_record(Answer, Proofs) :-
    arg(1, Answer, GoalId),
    arg(2, Answer, GoalAtom),
    arg(3, Answer, Outcome),
    ( generated_query_id(GoalId) ->
        validate_yes_no_outcome(Outcome, GoalAtom, Proofs)
    ; invariant(generated_yes_no_answer_shape)
    ).

validate_yes_no_outcome(Outcome, GoalAtom, Proofs) :-
    ( Outcome == proved ->
        validate_yes_no_proof(GoalAtom, Proofs)
    ; Outcome == not_proved ->
        ( Proofs == [] ->
            true
        ; invariant(generated_yes_no_proof_count)
        )
    ; invariant(generated_yes_no_outcome)
    ).

validate_yes_no_proof(GoalAtom, Proofs) :-
    ( has_functor(Proofs, '[|]', 2),
      arg(1, Proofs, Proof),
      arg(2, Proofs, Rest),
      Rest == [],
      ground(Proof),
      has_functor(Proof, proof, 3),
      arg(1, Proof, Root),
      Root == GoalAtom ->
        true
    ; invariant(generated_yes_no_proof)
    ).

answer_record_layout(Terms, Header, Document, Program, Answer, Proofs) :-
    list_head_tail(Terms, Header, Rest1),
    list_head_tail(Rest1, Document, Rest2),
    list_head_tail(Rest2, Program, Rest3),
    list_head_tail(Rest3, Answer, Proofs).

list_head_tail(List, Head, Tail) :-
    has_functor(List, '[|]', 2),
    arg(1, List, Head),
    arg(2, List, Tail).

validate_wh_answer_record(Header, Document, Program, Answer, Proofs) :-
    ( Header == cnl_answer_record(2),
      has_functor(Document, document, 3),
      generated_program_digest(Program),
      has_functor(Answer, answer, 4),
      arg(1, Answer, GoalId),
      generated_query_id(GoalId),
      arg(2, Answer, Marker),
      Marker == wh(who),
      arg(3, Answer, Pattern),
      generated_wh_pattern(Pattern, Name),
      arg(4, Answer, AnswerSet),
      has_functor(AnswerSet, answers, 1),
      arg(1, AnswerSet, Answers) ->
        validate_wh_answers(Answers, Name),
        validate_wh_proofs(Answers, Proofs)
    ; invariant(generated_wh_answer_shape)
    ).

generated_program_digest(Program) :-
    has_functor(Program, program, 1),
    arg(1, Program, Sha256),
    has_functor(Sha256, sha256, 1),
    arg(1, Sha256, Digest),
    atom(Digest).

generated_query_id(GoalId) :-
    has_functor(GoalId, query_id, 2),
    arg(1, GoalId, Sentence),
    arg(2, GoalId, Clause),
    has_functor(Sentence, sentence, 1),
    has_functor(Clause, clause, 1),
    arg(1, Sentence, S),
    arg(1, Clause, C),
    integer(S),
    S > 0,
    integer(C),
    C > 0.

generated_wh_pattern(Pattern, Name) :-
    has_functor(Pattern, pred, 2),
    arg(1, Pattern, Name),
    atom(Name),
    arg(2, Pattern, Args),
    has_functor(Args, '[|]', 2),
    arg(1, Args, Variable),
    arg(2, Args, Tail),
    Variable == var(1),
    Tail == [].

validate_wh_answers(Answers, _) :-
    Answers == [],
    !.
validate_wh_answers(Answers, Name) :-
    ( has_functor(Answers, '[|]', 2) ->
        arg(1, Answers, Atom),
        arg(2, Answers, Rest),
        ( generated_wh_answer_atom(Name, Atom),
          canonical_answer_key(Atom, Key) ->
            validate_wh_answer_tail(Rest, Name, Key)
        ; invariant(generated_wh_answers_shape)
        )
    ; invariant(generated_wh_answers_shape)
    ).

validate_wh_answer_tail(Answers, _, _) :-
    Answers == [],
    !.
validate_wh_answer_tail(Answers, Name, PreviousKey) :-
    ( has_functor(Answers, '[|]', 2) ->
        arg(1, Answers, Atom),
        arg(2, Answers, Rest),
        ( generated_wh_answer_atom(Name, Atom),
          canonical_answer_key(Atom, Key) ->
            ( PreviousKey @< Key ->
                validate_wh_answer_tail(Rest, Name, Key)
            ; invariant(generated_wh_answer_order)
            )
        ; invariant(generated_wh_answers_shape)
        )
    ; invariant(generated_wh_answers_shape)
    ).

generated_wh_answer_atom(Name, Atom) :-
    generated_ground_predicate(Atom),
    arg(1, Atom, StoredName),
    StoredName == Name,
    arg(2, Atom, Args),
    has_functor(Args, '[|]', 2),
    arg(2, Args, Tail),
    Tail == [].

generated_ground_predicate(Term) :-
    has_functor(Term, pred, 2),
    arg(1, Term, Name),
    atom(Name),
    arg(2, Term, Args),
    has_functor(Args, '[|]', 2),
    generated_ground_arguments(Args),
    ground(Term).

generated_ground_arguments(Args) :-
    ( Args == [] ->
        true
    ; has_functor(Args, '[|]', 2),
      arg(1, Args, Arg),
      arg(2, Args, Rest),
      replay_named_ground(Arg),
      generated_ground_arguments(Rest)
    ).

canonical_answer_key(Atom, Key) :-
    copy_term(Atom, Copy),
    ( catch(canonical_line(Copy, Line), _, fail) ->
        string_codes(Line, Key)
    ; invariant(generated_wh_answer_canonical)
    ).

validate_wh_proofs(Answers, Proofs) :-
    ( Answers == [] ->
        ( Proofs == [] ->
            true
        ; invariant(generated_wh_empty_proofs)
        )
    ; has_functor(Answers, '[|]', 2),
      has_functor(Proofs, '[|]', 2) ->
        arg(1, Answers, Atom),
        arg(2, Answers, AnswerRest),
        arg(1, Proofs, Proof),
        arg(2, Proofs, ProofRest),
        ( ground(Proof),
          has_functor(Proof, proof, 3),
          arg(1, Proof, Root),
          Root == Atom ->
            validate_wh_proofs(AnswerRest, ProofRest)
        ; invariant(generated_wh_proof_root)
        )
    ; invariant(generated_wh_proof_count)
    ).

build_certificate(Atom, Store, Proof) :-
    ( store_witness(Atom, Store, Witness) ->
        arg(1, Witness, ClauseId),
        arg(2, Witness, BodyEvidence),
        build_subproofs(BodyEvidence, Store, SubProofs),
        Proof = proof(Atom, ClauseId, SubProofs)
    ; invariant(missing_witness(Atom))
    ).

store_witness(Atom, [Entry|_], Witness) :-
    arg(1, Entry, Stored),
    Stored == Atom,
    arg(2, Entry, Witness),
    !.
store_witness(Atom, [_|Entries], Witness) :-
    store_witness(Atom, Entries, Witness).

build_subproofs(Evidence, _, []) :-
    Evidence == [],
    !.
build_subproofs(Evidence, Store, [SubProof|SubProofs]) :-
    has_functor(Evidence, '[|]', 2),
    arg(1, Evidence, Item),
    arg(2, Evidence, Rest),
    build_subproof(Item, Store, SubProof),
    build_subproofs(Rest, Store, SubProofs).

build_subproof(Item, _, naf(Atom)) :-
    has_functor(Item, naf, 1),
    arg(1, Item, Atom),
    replay_ground_predicate(Atom),
    !.
build_subproof(Item, Store, Proof) :-
    has_functor(Item, pred, 2),
    !,
    build_certificate(Item, Store, Proof).
build_subproof(Item, _, _) :-
    invariant(witness_body_item(Item)).

replay_certificate(GoalAtom, Proof, Store) :-
    ( has_functor(Proof, proof, 3),
      arg(1, Proof, RootAtom),
      RootAtom == GoalAtom,
      replay_node(Proof, Store) ->
        true
    ; invariant(replay_failed)
    ).

replay_node(Proof, Store) :-
    ground(Proof),
    has_functor(Proof, proof, 3),
    arg(1, Proof, NodeAtom),
    arg(2, Proof, ClauseId),
    arg(3, Proof, Children),
    program_clause_by_id(ClauseId, Head, Body),
    replay_match_predicate(Head, NodeAtom, [], Bindings1),
    replay_match_body(Body, Children, Bindings1, Bindings, Store),
    clause_variables(Head, Body, Variables),
    total_ground_bindings(Variables, Bindings),
    replay_substitute_predicate(Head, Bindings, GroundHead),
    replay_substitute_body(Body, Bindings, GroundBody),
    child_evidence(Children, ChildEvidence),
    GroundHead == NodeAtom,
    GroundBody == ChildEvidence,
    replay_children(Children, Store).

program_clause_by_id(ClauseId, Head, Body) :-
    cnl_program_db:program_clause(_, StoredId, StoredHead, StoredBody),
    StoredId == ClauseId,
    !,
    Head = StoredHead,
    Body = StoredBody.

child_evidence(Children, []) :-
    Children == [],
    !.
child_evidence(Children, [Evidence|EvidenceRest]) :-
    has_functor(Children, '[|]', 2),
    arg(1, Children, Child),
    arg(2, Children, Rest),
    child_item_evidence(Child, Evidence),
    child_evidence(Rest, EvidenceRest).

child_item_evidence(Child, Atom) :-
    has_functor(Child, proof, 3),
    arg(1, Child, Atom),
    !.
child_item_evidence(Child, Child) :-
    has_functor(Child, naf, 1),
    arg(1, Child, Atom),
    replay_ground_predicate(Atom).

replay_children(Children, _) :-
    Children == [],
    !.
replay_children(Children, Store) :-
    has_functor(Children, '[|]', 2),
    arg(1, Children, Child),
    arg(2, Children, Rest),
    replay_child(Child, Store),
    replay_children(Rest, Store).

replay_child(Child, Store) :-
    has_functor(Child, proof, 3),
    !,
    replay_node(Child, Store).
replay_child(Child, Store) :-
    has_functor(Child, naf, 1),
    arg(1, Child, Atom),
    replay_ground_predicate(Atom),
    replay_atom_absent(Atom, Store).

/* Independent structural matcher used only by the replay gate. */
replay_match_predicate(Pattern, Ground, Bindings0, Bindings) :-
    has_functor(Pattern, pred, 2),
    has_functor(Ground, pred, 2),
    arg(1, Pattern, PatternName),
    arg(2, Pattern, PatternArgs),
    arg(1, Ground, GroundName),
    arg(2, Ground, GroundArgs),
    PatternName == GroundName,
    replay_match_arguments(
        PatternArgs, GroundArgs, Bindings0, Bindings).

replay_match_arguments(PatternArgs, GroundArgs, Bindings, Bindings) :-
    PatternArgs == [],
    GroundArgs == [],
    !.
replay_match_arguments(PatternArgs, GroundArgs, Bindings0, Bindings) :-
    PatternArgs \== [],
    GroundArgs \== [],
    arg(1, PatternArgs, PatternArg),
    arg(2, PatternArgs, PatternRest),
    arg(1, GroundArgs, GroundArg),
    arg(2, GroundArgs, GroundRest),
    replay_match_argument(PatternArg, GroundArg, Bindings0, Bindings1),
    replay_match_arguments(
        PatternRest, GroundRest, Bindings1, Bindings).

replay_match_argument(Pattern, Ground, Bindings, Bindings) :-
    has_functor(Pattern, named, 1),
    Pattern == Ground,
    !.
replay_match_argument(Pattern, Ground, Bindings0, Bindings) :-
    has_functor(Pattern, var, 1),
    replay_named_ground(Ground),
    arg(1, Pattern, Number),
    ( replay_binding(Number, Bindings0, Bound) ->
        Bound == Ground,
        Bindings = Bindings0
    ; Bindings = [binding(Number, Ground)|Bindings0]
    ).

replay_match_body(Body, Children, Bindings, Bindings, _) :-
    Body == [],
    Children == [],
    !.
replay_match_body(Body, Children, Bindings0, Bindings, Store) :-
    has_functor(Body, '[|]', 2),
    has_functor(Children, '[|]', 2),
    arg(1, Body, Pattern),
    arg(2, Body, BodyRest),
    arg(1, Children, Child),
    arg(2, Children, ChildRest),
    replay_match_literal(Pattern, Child, Bindings0, Bindings1, Store),
    replay_match_body(
        BodyRest, ChildRest, Bindings1, Bindings, Store).

replay_match_literal(Pattern, Child, Bindings, Bindings, Store) :-
    has_functor(Pattern, naf, 1),
    !,
    has_functor(Child, naf, 1),
    arg(1, Pattern, NafPattern),
    arg(1, Child, Atom),
    replay_ground_predicate(Atom),
    replay_substitute_predicate(NafPattern, Bindings, Expected),
    Expected == Atom,
    replay_atom_absent(Atom, Store).
replay_match_literal(Pattern, Child, Bindings0, Bindings, _) :-
    has_functor(Child, proof, 3),
    arg(1, Child, Ground),
    replay_match_predicate(Pattern, Ground, Bindings0, Bindings).

clause_variables(Head, Body, Variables) :-
    replay_predicate_vars(Head, HeadVars),
    replay_body_vars(Body, BodyVars),
    append(HeadVars, BodyVars, Variables).

replay_predicate_vars(Predicate, Variables) :-
    arg(2, Predicate, Args),
    replay_argument_vars(Args, Variables, []).

replay_argument_vars(Args, Variables, Variables) :-
    Args == [],
    !.
replay_argument_vars(Args, Variables0, Variables) :-
    arg(1, Args, Arg),
    arg(2, Args, Rest),
    ( has_functor(Arg, var, 1) ->
        arg(1, Arg, Number),
        Variables0 = [Number|Tail]
    ; Tail = Variables0
    ),
    replay_argument_vars(Rest, Tail, Variables).

replay_body_vars(Body, []) :-
    Body == [],
    !.
replay_body_vars(Body, Variables) :-
    arg(1, Body, Literal),
    arg(2, Body, Rest),
    replay_literal_predicate(Literal, Predicate),
    replay_predicate_vars(Predicate, Here),
    replay_body_vars(Rest, Tail),
    append(Here, Tail, Variables).

replay_literal_predicate(Literal, Predicate) :-
    ( has_functor(Literal, naf, 1) ->
        arg(1, Literal, Predicate)
    ; Predicate = Literal
    ).

total_ground_bindings([], _).
total_ground_bindings([Number|Numbers], Bindings) :-
    replay_binding(Number, Bindings, Value),
    replay_named_ground(Value),
    ground(Value),
    total_ground_bindings(Numbers, Bindings).

replay_named_ground(Term) :-
    has_functor(Term, named, 1),
    arg(1, Term, Name),
    atom(Name).

replay_ground_predicate(Term) :-
    has_functor(Term, pred, 2),
    arg(1, Term, Name),
    arg(2, Term, Args),
    atom(Name),
    Args = [_|_],
    replay_ground_arguments(Args),
    ground(Term).

replay_ground_arguments([]).
replay_ground_arguments([Arg|Args]) :-
    replay_named_ground(Arg),
    replay_ground_arguments(Args).

replay_binding(Number, [binding(Here, Value)|_], Value) :-
    Here =:= Number,
    !.
replay_binding(Number, [_|Bindings], Value) :-
    replay_binding(Number, Bindings, Value).

replay_substitute_predicate(Pattern, Bindings, pred(Name, GroundArgs)) :-
    arg(1, Pattern, Name),
    arg(2, Pattern, Args),
    replay_substitute_arguments(Args, Bindings, GroundArgs).

replay_substitute_arguments(Args, _, []) :-
    Args == [],
    !.
replay_substitute_arguments(Args, Bindings, [Ground|Grounds]) :-
    arg(1, Args, Arg),
    arg(2, Args, Rest),
    replay_substitute_argument(Arg, Bindings, Ground),
    replay_substitute_arguments(Rest, Bindings, Grounds).

replay_substitute_argument(Arg, _, Arg) :-
    has_functor(Arg, named, 1),
    !.
replay_substitute_argument(Arg, Bindings, Ground) :-
    arg(1, Arg, Number),
    replay_binding(Number, Bindings, Ground).

replay_substitute_body(Body, _, []) :-
    Body == [],
    !.
replay_substitute_body(Body, Bindings, [Ground|Grounds]) :-
    arg(1, Body, Literal),
    arg(2, Body, Rest),
    replay_substitute_literal(Literal, Bindings, Ground),
    replay_substitute_body(Rest, Bindings, Grounds).

replay_substitute_literal(Literal, Bindings, naf(Ground)) :-
    has_functor(Literal, naf, 1),
    !,
    arg(1, Literal, Pattern),
    replay_substitute_predicate(Pattern, Bindings, Ground).
replay_substitute_literal(Pattern, Bindings, Ground) :-
    replay_substitute_predicate(Pattern, Bindings, Ground).

replay_atom_absent(Atom, Store) :-
    \+ replay_atom_present(Atom, Store).

replay_atom_present(Atom, [Entry|_]) :-
    arg(1, Entry, Stored),
    Stored == Atom,
    !.
replay_atom_present(Atom, [_|Entries]) :-
    replay_atom_present(Atom, Entries).

has_functor(Term, Name, Arity) :-
    compound(Term),
    functor(Term, Name, Arity).

invariant(Detail) :-
    throw(explanation_invariant(Detail)).
