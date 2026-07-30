:- module(ir_to_prolog, [compile_terms/2]).

:- set_prolog_flag(encoding, utf8).

:- use_module(ir_validate, [validate_terms/1]).
:- use_module(inference_kernel, [validate_program_terms/4]).

/*
IR v3 to program-record v3 compiler. Input has already passed framing gates.
Compilation is a total, order-preserving map on valid IR: provenance is dropped,
facts and rules become clauses, and the sole yes/no or wh query becomes the
final goal. Generated output is validated before it can reach the caller's
output buffer.
*/
compile_terms(IrTerms, ProgramTerms) :-
    validate_terms(IrTerms),
    transform_record(IrTerms, ProgramTerms),
    validate_generated_program(ProgramTerms).

validate_generated_program(ProgramTerms) :-
    catch(validate_program_terms(ProgramTerms, _, _, _),
        ir_reject(Class, Detail),
        throw(error(generated_record_invalid(Class, Detail),
            context(ir_to_prolog, program_validation)))).

transform_record([Header, Document|Items], ProgramTerms) :-
    Header == cnl_ir_record(3),
    transform_items(Items, ProgramItems),
    ProgramTerms = [cnl_program_record(3), Document|ProgramItems].

transform_items([], []).
transform_items([Item|Items], [ProgramItem|ProgramItems]) :-
    transform_item(Item, ProgramItem),
    transform_items(Items, ProgramItems).

transform_item(Item, clause(Id, Predicate, body([]))) :-
    compound(Item),
    functor(Item, fact, 3),
    arg(1, Item, Id),
    arg(2, Item, Predicate).
transform_item(Item, clause(Id, Head, Body)) :-
    compound(Item),
    functor(Item, rule, 4),
    arg(1, Item, Id),
    arg(2, Item, Head),
    arg(3, Item, Body).
transform_item(Item, Item) :-
    compound(Item),
    functor(Item, closed_world, 3).
transform_item(Item, alternative_set(Id, Members, Body,
        Satisfaction, Exclusivity, Exhaustiveness)) :-
    compound(Item),
    functor(Item, alternative_set, 7),
    arg(1, Item, Id),
    arg(2, Item, Members),
    arg(3, Item, Body),
    arg(4, Item, Satisfaction),
    arg(5, Item, Exclusivity),
    arg(6, Item, Exhaustiveness).
transform_item(Item, goal(Id, Predicate)) :-
    compound(Item),
    functor(Item, query, 3),
    arg(1, Item, Id),
    arg(2, Item, Predicate).
transform_item(Item, goal(Id, Marker, Predicate)) :-
    compound(Item),
    functor(Item, query, 4),
    arg(1, Item, Id),
    arg(2, Item, Marker),
    arg(3, Item, Predicate).
