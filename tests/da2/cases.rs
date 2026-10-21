pub const ULEX: &str = "noun_sg('opioid-dosage','opioid-dosage',neutr).\nnoun_pl('opioid-dosages','opioid-dosage',neutr).\nnoun_sg('oxycodone-milligram-per-day','oxycodone-milligram-per-day',neutr).\nnoun_pl('oxycodone-milligrams-per-day','oxycodone-milligram-per-day',neutr).\nnoun_sg('codeine-milligram-per-day','codeine-milligram-per-day',neutr).\nnoun_pl('codeine-milligrams-per-day','codeine-milligram-per-day',neutr).\nnoun_sg('fentanyl-microgram-per-hour','fentanyl-microgram-per-hour',neutr).\nnoun_pl('fentanyl-micrograms-per-hour','fentanyl-microgram-per-hour',neutr).\nnoun_sg('MME-unit-per-day','MME-unit-per-day',neutr).\nnoun_pl('MME-units-per-day','MME-unit-per-day',neutr).\nnoun_sg('covid-19-unit','covid-19-unit',neutr).\nnoun_pl('covid-19-units','covid-19-unit',neutr).\n";
pub const TABLE: &str = "# format: kind\tlemma\tvalue\nunit\tday\tday\nscope\trecommendation\tda2-scope\nexclusion\tapply\tscope\n";
pub const EXAMPLE: &str = "If an opioid-dosage contains some oxycodone-milligrams-per-day X then the opioid-dosage includes some MME-units-per-day Y and Y = X * 1.5.\n";
#[derive(Clone, Debug)]
pub struct Case {
    pub id: String,
    pub ace: String,
    pub family: &'static str,
    pub table: bool,
}
impl Case {
    pub fn new(id: impl Into<String>, ace: impl Into<String>, family: &'static str) -> Self {
        let mut ace = ace.into();
        if !ace.ends_with('\n') {
            ace.push('\n');
        }
        Self {
            id: id.into(),
            ace,
            family,
            table: false,
        }
    }
    pub fn table(&self) -> Option<&'static str> {
        self.table.then_some(TABLE)
    }
}
pub fn simple(id: &str, determiner: &str, expr: &str, family: &'static str) -> Case {
    let singular = determiner.ends_with(" 1") && !determiner.ends_with(" 11");
    let noun = if singular {
        "oxycodone-milligram-per-day"
    } else {
        "oxycodone-milligrams-per-day"
    };
    Case::new(
        id,
        format!(
            "If an opioid-dosage contains {determiner} {noun} X then the opioid-dosage includes some MME-units-per-day Y and Y = {expr}."
        ),
        family,
    )
}
pub fn accepted() -> Vec<Case> {
    let mut out = vec![
        Case::new("da2-product", EXAMPLE, "product"),
        simple("da2-quotient", "some", "X / 2", "quotient"),
        simple("da2-left-chain", "some", "(X * 3) / 2", "chain"),
        simple("da2-right-chain", "some", "X * (3 / 2) * 4.7", "chain"),
        Case::new(
            "da2-two-operands",
            "If an opioid-dosage contains some oxycodone-milligrams-per-day X and contains some codeine-milligrams-per-day Z then the opioid-dosage includes some MME-units-per-day Y and Y = X * Z.",
            "two-operands",
        ),
        Case::new(
            "da2-literal-only",
            "If there is an opioid-dosage then the opioid-dosage includes some MME-units-per-day Y and Y = 2 * 1.5.",
            "literal-only",
        ),
        Case::new(
            "da2-curried",
            "If there is an opioid-dosage then if the opioid-dosage contains some oxycodone-milligrams-per-day X then the opioid-dosage includes some MME-units-per-day Y and Y = X * 1.5.",
            "curried",
        ),
        Case::new(
            "da2-horn-shared",
            "If an opioid-dosage contains some oxycodone-milligrams-per-day X then if a man sleeps or a woman sleeps then the opioid-dosage includes some MME-units-per-day Y and Y = X * 1.5.",
            "horn",
        ),
        Case::new(
            "da2-two-targets",
            "If an opioid-dosage contains some oxycodone-milligrams-per-day X then the opioid-dosage includes some MME-units-per-day Y and Y = X * 1.5 and the opioid-dosage includes some codeine-milligrams-per-day Z and Z = X / 2.",
            "two-targets",
        ),
        Case::new(
            "da2-target-shared",
            "If an opioid-dosage contains some oxycodone-milligrams-per-day X then the opioid-dosage includes some MME-units-per-day Y and Y = X * 1.5 and the opioid-dosage sees Y.",
            "target-shared",
        ),
        Case::new(
            "da2-naf",
            "If an opioid-dosage contains some oxycodone-milligrams-per-day X and does not provably contain a cat then the opioid-dosage includes some MME-units-per-day Y and Y = X * 1.5.",
            "naf",
        ),
        Case::new(
            "da2-unrelated-some",
            "If an opioid-dosage contains some oxycodone-milligrams-per-day X and sees some cats then the opioid-dosage includes some MME-units-per-day Y and Y = X * 1.5.",
            "unrelated-some",
        ),
        Case::new(
            "da2-scoped-modal",
            "If an opioid-dosage contains some oxycodone-milligrams-per-day X then the opioid-dosage includes some MME-units-per-day Y and Y = X * 1.5 and a man should sleep.",
            "scoped-modal",
        ),
        simple("da2-literal-zero", "some", "X * 0", "literal"),
        simple("da2-literal-tenth", "some", "X * 0.15", "literal"),
        simple(
            "da2-literal-long",
            "some",
            "X * 123456789012345678901234567890",
            "literal",
        ),
        simple("da2-integer-before", "at least 50", "X * 2.4", "literal"),
        Case::new(
            "da2-digit-variable",
            EXAMPLE.replace(" X", " X1").replace("= X *", "= X1 *"),
            "literal",
        ),
        Case::new(
            "da2-digit-hyphen",
            EXAMPLE.replace("oxycodone-milligrams-per-day", "covid-19-units"),
            "literal",
        ),
        simple("da2-zero-lower-divisor", "at least 0", "2 / X", "witness"),
        simple(
            "da2-one-strict-upper-divisor",
            "less than 1",
            "2 / X",
            "witness",
        ),
        simple("da2-zero-upper-divisor", "at most 0", "2 / X", "witness"),
    ];
    out.iter_mut()
        .find(|p| p.id == "da2-scoped-modal")
        .unwrap()
        .table = true;
    for (cmp, word) in [
        ("exactly", "exactly"),
        ("geq", "at least"),
        ("greater", "more than"),
        ("leq", "at most"),
        ("less", "less than"),
    ] {
        for n in [0, 1, 2, 50] {
            out.push(simple(
                &format!("da2-bound-{cmp}-{n}"),
                &format!("{word} {n}"),
                "X * 1.5",
                "comparison",
            ));
        }
    }
    // Generated trees exercise fresh outputs, operand repetition, both node orders and association.
    let expressions = [
        "X * 0",
        "0 * X",
        "X * 1",
        "1 * X",
        "X / 2",
        "2 / X",
        "X * X",
        "(X * 3) / 2",
        "X * (3 / 2)",
        "(X / 2) * (X * 0.15)",
        "X / (2 / 3)",
        "(X * 4.7) / (5 * 0.15)",
    ];
    for (i, expr) in expressions.iter().enumerate() {
        for (j, det) in ["some", "exactly 2", "at least 0", "less than 1"]
            .iter()
            .enumerate()
        {
            out.push(simple(
                &format!("da2-generated-{i}-{j}"),
                det,
                expr,
                "generated",
            ));
        }
    }
    out
}
#[derive(Clone, Debug)]
pub struct Reject {
    pub case: Case,
    pub why: &'static str,
    pub mode: &'static str,
}
fn red(id: &str, ace: impl Into<String>, why: &'static str) -> Reject {
    Reject {
        case: Case::new(id, ace, "reject"),
        why,
        mode: "doc",
    }
}
pub fn rejected() -> Vec<Reject> {
    let mut out = vec![
        red(
            "da2-root",
            EXAMPLE
                .replace("If an", "An")
                .replace(" then the opioid-dosage", " and"),
            "position",
        ),
        red(
            "da2-antecedent",
            "If an opioid-dosage contains some oxycodone-milligrams-per-day X and X = 2 * 3 then the opioid-dosage sleeps.",
            "position",
        ),
        red(
            "da2-relation",
            EXAMPLE.replace("Y = X", "Y > X"),
            "relation",
        ),
        red(
            "da2-addition",
            EXAMPLE.replace("X * 1.5", "X + 2"),
            "operator",
        ),
        red(
            "da2-subtraction",
            EXAMPLE.replace("X * 1.5", "X - 2"),
            "operator",
        ),
        red(
            "da2-nested-operator",
            EXAMPLE.replace("X * 1.5", "(X + 1) * 2"),
            "operator",
        ),
        red(
            "da2-bare-variable",
            EXAMPLE.replace("X * 1.5", "X"),
            "operator",
        ),
        red(
            "da2-bare-literal",
            EXAMPLE.replace("X * 1.5", "1.5"),
            "operator",
        ),
        red(
            "da2-target-number",
            EXAMPLE.replace("some MME-units", "2 MME-units"),
            "target",
        ),
        red(
            "da2-target-comparison",
            EXAMPLE.replace("some MME-units", "at least 2 MME-units"),
            "target",
        ),
        red(
            "da2-target-adjective",
            EXAMPLE.replace("some MME-units", "some red MME-units"),
            "target",
        ),
        red(
            "da2-target-used",
            EXAMPLE.replace("X * 1.5", "Y * 1.5"),
            "target",
        ),
        red(
            "da2-nested-target",
            EXAMPLE.replace("X * 1.5", "X * (Y / 2)"),
            "target",
        ),
        red(
            "da2-target-repeated",
            EXAMPLE.replace("X * 1.5.", "X * 1.5 and Y = X / 2."),
            "target",
        ),
        red(
            "da2-other-target-operand",
            "If an opioid-dosage contains some oxycodone-milligrams-per-day X then the opioid-dosage includes some MME-units-per-day Y and Y = X * 1.5 and the opioid-dosage includes some codeine-milligrams-per-day Z and Z = Y / 2.",
            "target",
        ),
        red(
            "da2-target-nonref",
            EXAMPLE.replace("Y = X", "1 = X"),
            "target",
        ),
        red(
            "da2-modal-target",
            EXAMPLE.replace(
                "then the opioid-dosage includes",
                "then the opioid-dosage should include",
            ),
            "target",
        ),
        red(
            "da2-consequent-operand",
            "If there is an opioid-dosage then the opioid-dosage contains some oxycodone-milligrams-per-day X and includes some MME-units-per-day Y and Y = X * 1.5.",
            "operand",
        ),
        red(
            "da2-mass-operand",
            EXAMPLE.replace("some oxycodone-milligrams-per-day", "some water"),
            "operand",
        ),
        red(
            "da2-dom-operand",
            EXAMPLE.replace("some oxycodone-milligrams-per-day", "something"),
            "operand",
        ),
        red(
            "da2-time-operand",
            EXAMPLE.replace("some oxycodone-milligrams-per-day", "some days"),
            "operand",
        ),
        red(
            "da2-plain-operand",
            EXAMPLE.replace(
                "some oxycodone-milligrams-per-day",
                "2 oxycodone-milligrams-per-day",
            ),
            "operand_bound",
        ),
        red(
            "da2-indefinite-operand",
            EXAMPLE.replace(
                "some oxycodone-milligrams-per-day",
                "an oxycodone-milligram-per-day",
            ),
            "operand_bound",
        ),
        // reviewer-2 F1 + F2: a plain-number time quantity and an APE measurement are no operands.
        red(
            "da2-plain-time-operand",
            EXAMPLE
                .replace("some oxycodone-milligrams-per-day", "2 days")
                .replace("X * 1.5", "X * 2"),
            "operand",
        ),
        red(
            "da2-measured-operand",
            EXAMPLE
                .replace(
                    "some oxycodone-milligrams-per-day",
                    "at least 2 kg of apples",
                )
                .replace("X * 1.5", "X * 2"),
            "operand",
        ),
        red(
            "da2-measured-exact-operand",
            EXAMPLE
                .replace(
                    "some oxycodone-milligrams-per-day",
                    "exactly 2 kg of apples",
                )
                .replace("X * 1.5", "X * 2"),
            "operand",
        ),
        red(
            "da2-zero-divisor",
            EXAMPLE.replace("X * 1.5", "X / 0"),
            "witness",
        ),
        red(
            "da2-computed-zero-divisor",
            EXAMPLE.replace("X * 1.5", "X / (0 * X)"),
            "witness",
        ),
        red(
            "da2-inner-zero-divisor",
            EXAMPLE.replace("X * 1.5", "(X / 0) * 2"),
            "witness",
        ),
        red(
            "da2-literal-reject-antecedent",
            EXAMPLE.replace(
                "some oxycodone-milligrams-per-day",
                "at least 050 oxycodone-milligrams-per-day",
            ),
            "literal",
        ),
        red(
            "da2-zero-operand-divisor",
            simple("unused", "exactly 0", "1 / X", "reject").ace,
            "witness",
        ),
        red(
            "da2-order-position",
            "An opioid-dosage contains some oxycodone-milligrams-per-day X and includes 2 MME-units-per-day Y and Y > X + 1.50.",
            "position",
        ),
        red(
            "da2-order-relation",
            EXAMPLE
                .replace("Y = X * 1.5", "Y > X + 1.50")
                .replace("some MME-units", "2 MME-units"),
            "relation",
        ),
        red(
            "da2-order-operator",
            EXAMPLE
                .replace("X * 1.5", "X + 1.50")
                .replace("some MME-units", "2 MME-units"),
            "operator",
        ),
        red(
            "da2-order-target",
            EXAMPLE
                .replace("X * 1.5", "Y * 1.50")
                .replace("some MME-units", "2 MME-units"),
            "target",
        ),
        red(
            "da2-order-operand",
            EXAMPLE
                .replace("some oxycodone-milligrams-per-day", "some water")
                .replace("1.5", "1.50"),
            "operand",
        ),
        red(
            "da2-order-operand-after-bound",
            "If an opioid-dosage contains 2 oxycodone-milligrams-per-day X and contains some water Z then the opioid-dosage includes some MME-units-per-day Y and Y = X * Z.",
            "operand",
        ),
        red(
            "da2-order-bound",
            EXAMPLE
                .replace(
                    "some oxycodone-milligrams-per-day",
                    "2 oxycodone-milligrams-per-day",
                )
                .replace("1.5", "1.50"),
            "operand_bound",
        ),
        red(
            "da2-order-literal",
            EXAMPLE.replace("X * 1.5", "X / 00"),
            "literal",
        ),
        red(
            "da2-order-formulas-literal-first",
            "If an opioid-dosage contains some oxycodone-milligrams-per-day X then the opioid-dosage includes some MME-units-per-day Y and Y = X * 1.50 and the opioid-dosage includes some codeine-milligrams-per-day Z and Z = X + 2.",
            "literal",
        ),
        red(
            "da2-order-formulas-operator-first",
            "If an opioid-dosage contains some oxycodone-milligrams-per-day X then the opioid-dosage includes some MME-units-per-day Y and Y = X + 2 and the opioid-dosage includes some codeine-milligrams-per-day Z and Z = X * 1.50.",
            "operator",
        ),
    ];
    for id in ["da2-time-operand", "da2-plain-time-operand"] {
        out.iter_mut().find(|p| p.case.id == id).unwrap().case.table = true;
    }
    for (i, spelling) in [
        "1.50",
        "02",
        "0.10000000000000000000000001",
        "0.00001",
        "-1",
        "- 1",
        "(- 1.5)",
        "- 0",
        "50%",
    ]
    .iter()
    .enumerate()
    {
        out.push(red(
            &format!("da2-literal-reject-{i}"),
            EXAMPLE.replace("1.5", spelling),
            "literal",
        ));
    }
    out
}
