:- module(m6u3_review_gates, [
    quantity_matrix/0,
    replay_matrix/0,
    answer_self_check/0,
    resource_boundary/0
]).

:- use_module('../../src/prolog/validation_common').
:- use_module('../../src/prolog/explanation').
:- use_module('../../src/prolog/inference_kernel').
:- use_module('../../src/prolog/ir_tool').

quantity_matrix :-
    Qm2 = quantity(integer(-2),unit(score)),
    Q0 = quantity(integer(0),unit(score)),
    Q2 = quantity(integer(2),unit(score)),
    Q10 = quantity(integer(10),unit(score)),
    Q63 = quantity(integer(63),unit(year)),
    Q64 = quantity(integer(64),unit(year)),
    Q65 = quantity(integer(65),unit(year)),
    quantity_compare_outcome(Q64,quantity_bound(eq,closed,Q64),true),
    quantity_compare_outcome(Q63,quantity_bound(eq,closed,Q64),false),
    quantity_compare_outcome(Q65,quantity_bound(eq,closed,Q64),false),
    quantity_compare_outcome(Q65,quantity_bound(geq,closed,Q64),true),
    quantity_compare_outcome(Q63,quantity_bound(geq,closed,Q64),false),
    quantity_compare_outcome(Q65,quantity_bound(greater,open,Q64),true),
    quantity_compare_outcome(Q64,quantity_bound(greater,open,Q64),false),
    quantity_compare_outcome(Q63,quantity_bound(greater,open,Q64),false),
    quantity_compare_outcome(Q63,quantity_bound(leq,closed,Q64),true),
    quantity_compare_outcome(Q65,quantity_bound(leq,closed,Q64),false),
    quantity_compare_outcome(Q63,quantity_bound(less,open,Q64),true),
    quantity_compare_outcome(Q64,quantity_bound(less,open,Q64),false),
    quantity_compare_outcome(Q65,quantity_bound(less,open,Q64),false),
    quantity_compare_outcome(Q0,quantity_bound(eq,closed,Q0),true),
    quantity_compare_outcome(Qm2,quantity_bound(less,open,Q0),true),
    quantity_compare_outcome(Q2,quantity_bound(less,open,Q10),true),
    quantity_compare_outcome(Q10,quantity_bound(less,open,Q2),false).

replay_matrix :-
    setup_call_cleanup(
        retractall(cnl_program_db:program_clause(_,_,_,_)),
        replay_matrix_,
        retractall(cnl_program_db:program_clause(_,_,_,_))).

replay_matrix_ :-
    RuleId = rule_id(sentence(2),clause(1)),
    FactId = fact_id(sentence(1),clause(1)),
    Q = quantity(integer(70),unit(year)),
    Bound = quantity_bound(greater,open,quantity(integer(64),unit(year))),
    Head = pred(eligible,[var(1)]),
    Body = [pred(have,[var(1),var(2)]),quantity_compare(var(2),Bound)],
    assertz(cnl_program_db:program_clause(
        1,FactId,pred(have,[named(a),Q]),[])),
    assertz(cnl_program_db:program_clause(2,RuleId,Head,Body)),
    Have = pred(have,[named(a),Q]),
    Compare = quantity_compare(Q,Bound),
    Goal = pred(eligible,[named(a)]),
    Store = [
        entry(Have,by(FactId,[])),
        entry(Goal,by(RuleId,[Have,Compare]))
    ],
    HaveProof = proof(Have,FactId,[]),
    Proof = proof(Goal,RuleId,[HaveProof,Compare]),
    explanation:replay_certificate(Goal,Proof,Store),
    replay_reject(Goal,proof(Goal,RuleId,[HaveProof,
        quantity_compare(quantity(integer(71),unit(year)),Bound)]),Store),
    replay_reject(Goal,proof(Goal,RuleId,[HaveProof,
        quantity_compare(quantity(integer(70),unit(day)),Bound)]),Store),
    replay_reject(Goal,proof(Goal,RuleId,[HaveProof,
        quantity_compare(Q,quantity_bound(geq,closed,
            quantity(integer(64),unit(year))))]),Store),
    replay_reject(Goal,proof(Goal,RuleId,[HaveProof,
        quantity_compare(Q,quantity_bound(greater,closed,
            quantity(integer(64),unit(year))))]),Store),
    replay_reject(Goal,proof(Goal,RuleId,[HaveProof,
        quantity_compare(Q,quantity_bound(greater,open,
            quantity(integer(65),unit(year))))]),Store),
    replay_reject(Goal,proof(Goal,RuleId,[HaveProof,
        quantity_compare(Q,quantity_bound(greater,open,
            quantity(integer(64),unit(day))))]),Store),
    replay_reject(Goal,proof(Goal,RuleId,[HaveProof]),Store),
    replay_reject(Goal,proof(Goal,RuleId,[HaveProof,Compare,Compare]),Store),
    replay_reject(Goal,proof(Goal,RuleId,[HaveProof,
        quantity_compare(Bound,Q)]),Store),
    replay_two_guard_order.

replay_two_guard_order :-
    RuleId = rule_id(sentence(3),clause(1)),
    Q = quantity(integer(70),unit(year)),
    Lower = quantity_bound(greater,open,quantity(integer(64),unit(year))),
    Upper = quantity_bound(less,open,quantity(integer(80),unit(year))),
    C1 = quantity_compare(Q,Lower),
    C2 = quantity_compare(Q,Upper),
    Goal = pred(two_guards,[named(a)]),
    assertz(cnl_program_db:program_clause(3,RuleId,Goal,[C1,C2])),
    Store = [entry(Goal,by(RuleId,[C1,C2]))],
    explanation:replay_certificate(
        Goal,proof(Goal,RuleId,[C1,C2]),Store),
    replay_reject(Goal,proof(Goal,RuleId,[C2,C1]),Store),
    replay_reject(Goal,proof(Goal,RuleId,[C1,C1]),Store).

replay_reject(Goal, Proof, Store) :-
    catch(explanation:replay_certificate(Goal,Proof,Store),Error,true),
    Error == explanation_invariant(replay_failed).

answer_self_check :-
    Doc = document(docid(d),source_sha256(s),ulex(none)),
    Program = program(sha256(h)),
    Id = query_id(sentence(1),clause(1)),
    Good = pred(p,[quantity(integer(1),unit(mg))]),
    GoodProof = proof(Good,fact_id(sentence(1),clause(1)),[]),
    explanation:validate_answer_terms([
        cnl_answer_record(3),Doc,Program,answer(Id,Good,proved),GoodProof]),
    Bad = pred(p,[var(1)]),
    BadProof = proof(Bad,fact_id(sentence(1),clause(1)),[]),
    BadTerms = [
        cnl_answer_record(3),Doc,Program,answer(Id,Bad,proved),BadProof],
    catch(explanation:validate_answer_terms(BadTerms),ExportedError,true),
    ExportedError == explanation_invariant(generated_yes_no_answer_shape),
    catch(ir_tool:self_checked_answer_codes(BadTerms,_),ToolError,true),
    ToolError == explanation_invariant(generated_yes_no_answer_shape).

resource_boundary :-
    Q = quantity(integer(70),unit(year)),
    C1 = quantity_compare(Q,
        quantity_bound(greater,open,quantity(integer(64),unit(year)))),
    C2 = quantity_compare(Q,
        quantity_bound(less,open,quantity(integer(80),unit(year)))),
    Accepted = pred(cap,[named(accepted)]),
    AcceptedStore = [entry(Accepted,by(
        rule_id(sentence(1),clause(1)),[C1]))],
    inference_kernel:certificate_preflight_with_cap(
        [Accepted],AcceptedStore,2),
    Rejected = pred(cap,[named(rejected)]),
    RejectedStore = [entry(Rejected,by(
        rule_id(sentence(2),clause(1)),[C1,C2]))],
    catch(inference_kernel:certificate_preflight_with_cap(
        [Rejected],RejectedStore,2),Error,true),
    Error == ir_reject(resource,certificate_nodes_exceed_cap(2)).
