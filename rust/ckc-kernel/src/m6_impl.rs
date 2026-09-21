// M6 document + query certification.
use ckc_spec::replay::EOut;
use vstd::prelude::*;

verus! {

pub fn certify_doc_impl(
    ace: &[u8],
    asha: &[u8],
    usha: Option<&Vec<u8>>,
    docid: &[u8],
    dump: &[u8],
    pl: &[u8],
) -> (r: EOut)
    ensures
        r@ == ckc_spec::emit::certify_doc_output(
            ace@,
            asha@,
            ckc_spec::emit::opt_view(usha),
            docid@,
            dump@,
            pl@,
        ),
{
    crate::m6_doc::certify_doc_impl(ace, asha, usha, docid, dump, pl)
}

pub fn certify_query_impl(
    ace: &[u8],
    asha: &[u8],
    usha: Option<&Vec<u8>>,
    qid: &[u8],
    dump: &[u8],
    pl: &[u8],
) -> (r: EOut)
    ensures
        r@ == ckc_spec::emit::certify_query_output(
            ace@,
            asha@,
            ckc_spec::emit::opt_view(usha),
            qid@,
            dump@,
            pl@,
        ),
{
    crate::m6_query::certify_query_impl(ace, asha, usha, qid, dump, pl)
}

} // verus!
