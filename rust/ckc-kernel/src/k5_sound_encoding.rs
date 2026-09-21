use crate::k5_sound_chrome as ch;
use ckc_spec::ui as u;
use vstd::prelude::*;
use vstd::utf8::*;
#[cfg(verus_keep_ghost)]
macro_rules! reveal_css_text { () => { reveal_strlit(r##"body { margin: 0 auto; max-width: 72rem; padding: 0 1.5rem 4rem; font-family: system-ui, sans-serif; line-height: 1.55; color: #111827; background: #ffffff; }
a { color: #1d4ed8; }
a:focus-visible, summary:focus-visible { outline: 3px solid #1d4ed8; outline-offset: 2px; }
.skip { position: absolute; left: -999px; top: 0; padding: 0.5rem 1rem; background: #ffffff; color: #1d4ed8; }
.skip:focus { left: 0; z-index: 1; }
nav.crumbs { padding: 1rem 0; border-bottom: 1px solid #e5e7eb; }
h1 { font-size: 1.5rem; }
h2 { font-size: 1.25rem; }
h3 { font-size: 1.05rem; }
h1 a.source { font-size: 1rem; font-weight: 400; margin-left: 0.5rem; }
table { border-collapse: collapse; width: 100%; margin: 1rem 0; }
th, td { text-align: left; padding: 0.4rem 0.6rem; border-bottom: 1px solid #e5e7eb; vertical-align: top; }
th { border-bottom: 2px solid #111827; }
table.compact { width: auto; }
table.compact th, table.compact td { padding-right: 2rem; }
table.records { table-layout: fixed; }
table.records th { box-sizing: border-box; }
table.records th:nth-child(1) { width: 12%; }
table.records th:nth-child(2) { width: 18%; }
table.records th:nth-child(3) { width: 22%; }
table.records th:nth-child(4) { width: 10%; }
.chip { display: inline-block; padding: 0.1rem 0.6rem; border-radius: 999px; font-size: 0.85rem; font-weight: 600; }
.chip-approved { color: #14532d; background: #dcfce7; }
.chip-rejected { color: #7f1d1d; background: #fee2e2; }
.chip-contested { color: #4c1d95; background: #ede9fe; }
.chip-stale { color: #78350f; background: #fef3c7; }
.chip-unreviewed { color: #1f2937; background: #e5e7eb; }
pre { padding: 0.75rem 1rem; border: 1px solid #e5e7eb; white-space: pre-wrap; overflow-x: auto; }
pre, code { font-family: ui-monospace, Menlo, Consolas, monospace; font-size: 0.95rem; }
pre.prose { font-family: Georgia, serif; font-size: 1.05rem; overflow-wrap: anywhere; }
dt { font-weight: 600; margin-top: 0.6rem; }
dd { margin-left: 0; }
summary { cursor: pointer; }
section { margin: 1.5rem 0; }
nav.docnav { padding: 1rem 0; border-top: 1px solid #e5e7eb; }
footer.scope { margin-top: 2rem; padding: 1rem 0; border-top: 1px solid #e5e7eb; font-size: 0.9rem; }
form label { display: block; margin-top: 1rem; font-weight: 600; }
fieldset { border: 0; margin: 1rem 0 0; padding: 0; max-width: 28rem; }
legend { font-weight: 600; padding: 0; }
fieldset label { margin-top: 0.5rem; font-weight: 400; }
input[type="text"], textarea { display: block; box-sizing: border-box; width: 100%; max-width: 28rem; margin-top: 0.3rem; padding: 0.45rem 0.6rem; border: 1px solid #e5e7eb; font-family: inherit; font-size: 1rem; color: #111827; background: #ffffff; }
textarea { min-height: 6rem; }
input[type="radio"], input[type="checkbox"] { accent-color: #111827; }
input[type="text"]:focus-visible, input[type="radio"]:focus-visible, input[type="checkbox"]:focus-visible, textarea:focus-visible, button:focus-visible { outline: 3px solid #1d4ed8; outline-offset: 2px; }
button { margin-top: 1.25rem; padding: 0.5rem 1.2rem; border: 1px solid #111827; font-family: inherit; font-size: 1rem; font-weight: 600; color: #ffffff; background: #111827; }
mark { background: #dbeafe; color: inherit; text-decoration: underline dotted #4b5563; text-underline-offset: 0.15em; }
.kw { color: #4b5563; }
.hl-note { color: #4b5563; font-size: 0.9rem; }
mark.t1, mark.t13, mark.t25, mark.t37 { background: #fef9c3; }
mark.t2, mark.t14, mark.t26, mark.t38 { background: #f3e8ff; }
mark.t3, mark.t15, mark.t27, mark.t39 { background: #ffedd5; }
mark.t4, mark.t16, mark.t28, mark.t40 { background: #ccfbf1; }
mark.t5, mark.t17, mark.t29, mark.t41 { background: #ffe4e6; }
mark.t6, mark.t18, mark.t30, mark.t42 { background: #dcfce7; }
mark.t7, mark.t19, mark.t31, mark.t43 { background: #fae8ff; }
mark.t8, mark.t20, mark.t32, mark.t44 { background: #cffafe; }
mark.t9, mark.t21, mark.t33, mark.t45 { background: #ecfccb; }
mark.t10, mark.t22, mark.t34, mark.t46 { background: #e0e7ff; }
mark.t11, mark.t23, mark.t35, mark.t47 { background: #e7e5e4; }
mark.t0, mark.t12, mark.t24, mark.t36 { text-decoration-color: #2563eb; }
mark.t1, mark.t13, mark.t25, mark.t37 { text-decoration-color: #a16207; }
mark.t2, mark.t14, mark.t26, mark.t38 { text-decoration-color: #7c3aed; }
mark.t3, mark.t15, mark.t27, mark.t39 { text-decoration-color: #c2410c; }
mark.t4, mark.t16, mark.t28, mark.t40 { text-decoration-color: #0f766e; }
mark.t5, mark.t17, mark.t29, mark.t41 { text-decoration-color: #be123c; }
mark.t6, mark.t18, mark.t30, mark.t42 { text-decoration-color: #15803d; }
mark.t7, mark.t19, mark.t31, mark.t43 { text-decoration-color: #a21caf; }
mark.t8, mark.t20, mark.t32, mark.t44 { text-decoration-color: #0e7490; }
mark.t9, mark.t21, mark.t33, mark.t45 { text-decoration-color: #4d7c0f; }
mark.t10, mark.t22, mark.t34, mark.t46 { text-decoration-color: #4f46e5; }
mark.t11, mark.t23, mark.t35, mark.t47 { text-decoration-color: #57534e; }
main:has(mark.t0:hover) mark.t0 { background: #bfdbfe; text-decoration-style: solid; }
main:has(mark.t1:hover) mark.t1 { background: #fef08a; text-decoration-style: solid; }
main:has(mark.t2:hover) mark.t2 { background: #e9d5ff; text-decoration-style: solid; }
main:has(mark.t3:hover) mark.t3 { background: #fed7aa; text-decoration-style: solid; }
main:has(mark.t4:hover) mark.t4 { background: #99f6e4; text-decoration-style: solid; }
main:has(mark.t5:hover) mark.t5 { background: #fecdd3; text-decoration-style: solid; }
main:has(mark.t6:hover) mark.t6 { background: #bbf7d0; text-decoration-style: solid; }
main:has(mark.t7:hover) mark.t7 { background: #f5d0fe; text-decoration-style: solid; }
main:has(mark.t8:hover) mark.t8 { background: #a5f3fc; text-decoration-style: solid; }
main:has(mark.t9:hover) mark.t9 { background: #d9f99d; text-decoration-style: solid; }
main:has(mark.t10:hover) mark.t10 { background: #c7d2fe; text-decoration-style: solid; }
main:has(mark.t11:hover) mark.t11 { background: #d6d3d1; text-decoration-style: solid; }
main:has(mark.t12:hover) mark.t12 { background: #bfdbfe; text-decoration-style: solid; }
main:has(mark.t13:hover) mark.t13 { background: #fef08a; text-decoration-style: solid; }
main:has(mark.t14:hover) mark.t14 { background: #e9d5ff; text-decoration-style: solid; }
main:has(mark.t15:hover) mark.t15 { background: #fed7aa; text-decoration-style: solid; }
main:has(mark.t16:hover) mark.t16 { background: #99f6e4; text-decoration-style: solid; }
main:has(mark.t17:hover) mark.t17 { background: #fecdd3; text-decoration-style: solid; }
main:has(mark.t18:hover) mark.t18 { background: #bbf7d0; text-decoration-style: solid; }
main:has(mark.t19:hover) mark.t19 { background: #f5d0fe; text-decoration-style: solid; }
main:has(mark.t20:hover) mark.t20 { background: #a5f3fc; text-decoration-style: solid; }
main:has(mark.t21:hover) mark.t21 { background: #d9f99d; text-decoration-style: solid; }
main:has(mark.t22:hover) mark.t22 { background: #c7d2fe; text-decoration-style: solid; }
main:has(mark.t23:hover) mark.t23 { background: #d6d3d1; text-decoration-style: solid; }
main:has(mark.t24:hover) mark.t24 { background: #bfdbfe; text-decoration-style: solid; }
main:has(mark.t25:hover) mark.t25 { background: #fef08a; text-decoration-style: solid; }
main:has(mark.t26:hover) mark.t26 { background: #e9d5ff; text-decoration-style: solid; }
main:has(mark.t27:hover) mark.t27 { background: #fed7aa; text-decoration-style: solid; }
main:has(mark.t28:hover) mark.t28 { background: #99f6e4; text-decoration-style: solid; }
main:has(mark.t29:hover) mark.t29 { background: #fecdd3; text-decoration-style: solid; }
main:has(mark.t30:hover) mark.t30 { background: #bbf7d0; text-decoration-style: solid; }
main:has(mark.t31:hover) mark.t31 { background: #f5d0fe; text-decoration-style: solid; }
main:has(mark.t32:hover) mark.t32 { background: #a5f3fc; text-decoration-style: solid; }
main:has(mark.t33:hover) mark.t33 { background: #d9f99d; text-decoration-style: solid; }
main:has(mark.t34:hover) mark.t34 { background: #c7d2fe; text-decoration-style: solid; }
main:has(mark.t35:hover) mark.t35 { background: #d6d3d1; text-decoration-style: solid; }
main:has(mark.t36:hover) mark.t36 { background: #bfdbfe; text-decoration-style: solid; }
main:has(mark.t37:hover) mark.t37 { background: #fef08a; text-decoration-style: solid; }
main:has(mark.t38:hover) mark.t38 { background: #e9d5ff; text-decoration-style: solid; }
main:has(mark.t39:hover) mark.t39 { background: #fed7aa; text-decoration-style: solid; }
main:has(mark.t40:hover) mark.t40 { background: #99f6e4; text-decoration-style: solid; }
main:has(mark.t41:hover) mark.t41 { background: #fecdd3; text-decoration-style: solid; }
main:has(mark.t42:hover) mark.t42 { background: #bbf7d0; text-decoration-style: solid; }
main:has(mark.t43:hover) mark.t43 { background: #f5d0fe; text-decoration-style: solid; }
main:has(mark.t44:hover) mark.t44 { background: #a5f3fc; text-decoration-style: solid; }
main:has(mark.t45:hover) mark.t45 { background: #d9f99d; text-decoration-style: solid; }
main:has(mark.t46:hover) mark.t46 { background: #c7d2fe; text-decoration-style: solid; }
main:has(mark.t47:hover) mark.t47 { background: #d6d3d1; text-decoration-style: solid; }
.hl-note label { margin-right: 0.75rem; }
main.hl-click pre.prose { color: #9ca3af; }
main.hl-click pre.prose .kw { color: #9ca3af; }
main.hl-click pre.prose mark:not(.hl-pick) { background: none; text-decoration-color: #9ca3af; }
main.hl-click pre.prose mark.hl-pick { color: #111827; }
body:has(input.hl-toggle:not(:checked)) pre.prose mark { background: none; text-decoration: none; color: inherit; }
body:has(input.hl-toggle:not(:checked)) pre.prose .kw { color: inherit; }
@media print {
body { max-width: none; padding: 0; }
nav.crumbs, nav.docnav, .skip, form, .verdict-entry, .hl-note { display: none; }
mark, mark[class] { background: none; text-decoration-color: #4b5563; }
main.hl-click pre.prose, main.hl-click pre.prose mark.hl-pick { color: inherit; }
main.hl-click pre.prose .kw { color: #4b5563; }
main.hl-click pre.prose mark:not(.hl-pick) { text-decoration-color: #4b5563; }
details::details-content { content-visibility: visible; }
pre { border: none; padding: 0; white-space: pre-wrap; overflow-x: visible; }
a { color: inherit; text-decoration: none; }
.chip { border: 1px solid #111827; background: none; color: inherit; }
}"##) }; }
#[cfg(verus_keep_ghost)]
macro_rules! reveal_script_html {
    () => {
        reveal_strlit(
            r##"<script>
(function () {
"use strict";
var picked = "";
function groupOf(node) {
if (node === null) { return ""; }
var m = node.closest("mark");
if (m === null) { return ""; }
var name = m.classList.item(0);
if (name === null) { return ""; }
return name;
}
function clearPick() {
if (picked === "") { return; }
document.getElementById("main").classList.remove("hl-click");
var marks = document.querySelectorAll("mark.hl-pick");
var i = 0;
while (i < marks.length) { marks[i].classList.remove("hl-pick"); i += 1; }
picked = "";
}
function toggleBox() { return document.querySelector("input.hl-toggle"); }
document.addEventListener("click", function (ev) {
var name = groupOf(ev.target);
if (name === "") { return; }
var box = toggleBox();
if (box === null) { return; }
if (box.checked === false) { return; }
if (picked === name) { clearPick(); return; }
clearPick();
var marks = document.querySelectorAll("mark." + name);
var i = 0;
while (i < marks.length) { marks[i].classList.add("hl-pick"); i += 1; }
document.getElementById("main").classList.add("hl-click");
picked = name;
});
document.addEventListener("mouseout", function (ev) {
if (picked === "") { return; }
if (groupOf(ev.target) !== picked) { return; }
if (groupOf(ev.relatedTarget) === picked) { return; }
clearPick();
});
document.addEventListener("change", function (ev) {
var box = toggleBox();
if (box === null) { return; }
if (ev.target === box) { if (box.checked === false) { clearPick(); } }
});
})();
</script>"##,
        )
    };
}
verus! {

pub open spec fn css_text_chars() -> Seq<char> {
    r##"body { margin: 0 auto; max-width: 72rem; padding: 0 1.5rem 4rem; font-family: system-ui, sans-serif; line-height: 1.55; color: #111827; background: #ffffff; }
a { color: #1d4ed8; }
a:focus-visible, summary:focus-visible { outline: 3px solid #1d4ed8; outline-offset: 2px; }
.skip { position: absolute; left: -999px; top: 0; padding: 0.5rem 1rem; background: #ffffff; color: #1d4ed8; }
.skip:focus { left: 0; z-index: 1; }
nav.crumbs { padding: 1rem 0; border-bottom: 1px solid #e5e7eb; }
h1 { font-size: 1.5rem; }
h2 { font-size: 1.25rem; }
h3 { font-size: 1.05rem; }
h1 a.source { font-size: 1rem; font-weight: 400; margin-left: 0.5rem; }
table { border-collapse: collapse; width: 100%; margin: 1rem 0; }
th, td { text-align: left; padding: 0.4rem 0.6rem; border-bottom: 1px solid #e5e7eb; vertical-align: top; }
th { border-bottom: 2px solid #111827; }
table.compact { width: auto; }
table.compact th, table.compact td { padding-right: 2rem; }
table.records { table-layout: fixed; }
table.records th { box-sizing: border-box; }
table.records th:nth-child(1) { width: 12%; }
table.records th:nth-child(2) { width: 18%; }
table.records th:nth-child(3) { width: 22%; }
table.records th:nth-child(4) { width: 10%; }
.chip { display: inline-block; padding: 0.1rem 0.6rem; border-radius: 999px; font-size: 0.85rem; font-weight: 600; }
.chip-approved { color: #14532d; background: #dcfce7; }
.chip-rejected { color: #7f1d1d; background: #fee2e2; }
.chip-contested { color: #4c1d95; background: #ede9fe; }
.chip-stale { color: #78350f; background: #fef3c7; }
.chip-unreviewed { color: #1f2937; background: #e5e7eb; }
pre { padding: 0.75rem 1rem; border: 1px solid #e5e7eb; white-space: pre-wrap; overflow-x: auto; }
pre, code { font-family: ui-monospace, Menlo, Consolas, monospace; font-size: 0.95rem; }
pre.prose { font-family: Georgia, serif; font-size: 1.05rem; overflow-wrap: anywhere; }
dt { font-weight: 600; margin-top: 0.6rem; }
dd { margin-left: 0; }
summary { cursor: pointer; }
section { margin: 1.5rem 0; }
nav.docnav { padding: 1rem 0; border-top: 1px solid #e5e7eb; }
footer.scope { margin-top: 2rem; padding: 1rem 0; border-top: 1px solid #e5e7eb; font-size: 0.9rem; }
form label { display: block; margin-top: 1rem; font-weight: 600; }
fieldset { border: 0; margin: 1rem 0 0; padding: 0; max-width: 28rem; }
legend { font-weight: 600; padding: 0; }
fieldset label { margin-top: 0.5rem; font-weight: 400; }
input[type="text"], textarea { display: block; box-sizing: border-box; width: 100%; max-width: 28rem; margin-top: 0.3rem; padding: 0.45rem 0.6rem; border: 1px solid #e5e7eb; font-family: inherit; font-size: 1rem; color: #111827; background: #ffffff; }
textarea { min-height: 6rem; }
input[type="radio"], input[type="checkbox"] { accent-color: #111827; }
input[type="text"]:focus-visible, input[type="radio"]:focus-visible, input[type="checkbox"]:focus-visible, textarea:focus-visible, button:focus-visible { outline: 3px solid #1d4ed8; outline-offset: 2px; }
button { margin-top: 1.25rem; padding: 0.5rem 1.2rem; border: 1px solid #111827; font-family: inherit; font-size: 1rem; font-weight: 600; color: #ffffff; background: #111827; }
mark { background: #dbeafe; color: inherit; text-decoration: underline dotted #4b5563; text-underline-offset: 0.15em; }
.kw { color: #4b5563; }
.hl-note { color: #4b5563; font-size: 0.9rem; }
mark.t1, mark.t13, mark.t25, mark.t37 { background: #fef9c3; }
mark.t2, mark.t14, mark.t26, mark.t38 { background: #f3e8ff; }
mark.t3, mark.t15, mark.t27, mark.t39 { background: #ffedd5; }
mark.t4, mark.t16, mark.t28, mark.t40 { background: #ccfbf1; }
mark.t5, mark.t17, mark.t29, mark.t41 { background: #ffe4e6; }
mark.t6, mark.t18, mark.t30, mark.t42 { background: #dcfce7; }
mark.t7, mark.t19, mark.t31, mark.t43 { background: #fae8ff; }
mark.t8, mark.t20, mark.t32, mark.t44 { background: #cffafe; }
mark.t9, mark.t21, mark.t33, mark.t45 { background: #ecfccb; }
mark.t10, mark.t22, mark.t34, mark.t46 { background: #e0e7ff; }
mark.t11, mark.t23, mark.t35, mark.t47 { background: #e7e5e4; }
mark.t0, mark.t12, mark.t24, mark.t36 { text-decoration-color: #2563eb; }
mark.t1, mark.t13, mark.t25, mark.t37 { text-decoration-color: #a16207; }
mark.t2, mark.t14, mark.t26, mark.t38 { text-decoration-color: #7c3aed; }
mark.t3, mark.t15, mark.t27, mark.t39 { text-decoration-color: #c2410c; }
mark.t4, mark.t16, mark.t28, mark.t40 { text-decoration-color: #0f766e; }
mark.t5, mark.t17, mark.t29, mark.t41 { text-decoration-color: #be123c; }
mark.t6, mark.t18, mark.t30, mark.t42 { text-decoration-color: #15803d; }
mark.t7, mark.t19, mark.t31, mark.t43 { text-decoration-color: #a21caf; }
mark.t8, mark.t20, mark.t32, mark.t44 { text-decoration-color: #0e7490; }
mark.t9, mark.t21, mark.t33, mark.t45 { text-decoration-color: #4d7c0f; }
mark.t10, mark.t22, mark.t34, mark.t46 { text-decoration-color: #4f46e5; }
mark.t11, mark.t23, mark.t35, mark.t47 { text-decoration-color: #57534e; }
main:has(mark.t0:hover) mark.t0 { background: #bfdbfe; text-decoration-style: solid; }
main:has(mark.t1:hover) mark.t1 { background: #fef08a; text-decoration-style: solid; }
main:has(mark.t2:hover) mark.t2 { background: #e9d5ff; text-decoration-style: solid; }
main:has(mark.t3:hover) mark.t3 { background: #fed7aa; text-decoration-style: solid; }
main:has(mark.t4:hover) mark.t4 { background: #99f6e4; text-decoration-style: solid; }
main:has(mark.t5:hover) mark.t5 { background: #fecdd3; text-decoration-style: solid; }
main:has(mark.t6:hover) mark.t6 { background: #bbf7d0; text-decoration-style: solid; }
main:has(mark.t7:hover) mark.t7 { background: #f5d0fe; text-decoration-style: solid; }
main:has(mark.t8:hover) mark.t8 { background: #a5f3fc; text-decoration-style: solid; }
main:has(mark.t9:hover) mark.t9 { background: #d9f99d; text-decoration-style: solid; }
main:has(mark.t10:hover) mark.t10 { background: #c7d2fe; text-decoration-style: solid; }
main:has(mark.t11:hover) mark.t11 { background: #d6d3d1; text-decoration-style: solid; }
main:has(mark.t12:hover) mark.t12 { background: #bfdbfe; text-decoration-style: solid; }
main:has(mark.t13:hover) mark.t13 { background: #fef08a; text-decoration-style: solid; }
main:has(mark.t14:hover) mark.t14 { background: #e9d5ff; text-decoration-style: solid; }
main:has(mark.t15:hover) mark.t15 { background: #fed7aa; text-decoration-style: solid; }
main:has(mark.t16:hover) mark.t16 { background: #99f6e4; text-decoration-style: solid; }
main:has(mark.t17:hover) mark.t17 { background: #fecdd3; text-decoration-style: solid; }
main:has(mark.t18:hover) mark.t18 { background: #bbf7d0; text-decoration-style: solid; }
main:has(mark.t19:hover) mark.t19 { background: #f5d0fe; text-decoration-style: solid; }
main:has(mark.t20:hover) mark.t20 { background: #a5f3fc; text-decoration-style: solid; }
main:has(mark.t21:hover) mark.t21 { background: #d9f99d; text-decoration-style: solid; }
main:has(mark.t22:hover) mark.t22 { background: #c7d2fe; text-decoration-style: solid; }
main:has(mark.t23:hover) mark.t23 { background: #d6d3d1; text-decoration-style: solid; }
main:has(mark.t24:hover) mark.t24 { background: #bfdbfe; text-decoration-style: solid; }
main:has(mark.t25:hover) mark.t25 { background: #fef08a; text-decoration-style: solid; }
main:has(mark.t26:hover) mark.t26 { background: #e9d5ff; text-decoration-style: solid; }
main:has(mark.t27:hover) mark.t27 { background: #fed7aa; text-decoration-style: solid; }
main:has(mark.t28:hover) mark.t28 { background: #99f6e4; text-decoration-style: solid; }
main:has(mark.t29:hover) mark.t29 { background: #fecdd3; text-decoration-style: solid; }
main:has(mark.t30:hover) mark.t30 { background: #bbf7d0; text-decoration-style: solid; }
main:has(mark.t31:hover) mark.t31 { background: #f5d0fe; text-decoration-style: solid; }
main:has(mark.t32:hover) mark.t32 { background: #a5f3fc; text-decoration-style: solid; }
main:has(mark.t33:hover) mark.t33 { background: #d9f99d; text-decoration-style: solid; }
main:has(mark.t34:hover) mark.t34 { background: #c7d2fe; text-decoration-style: solid; }
main:has(mark.t35:hover) mark.t35 { background: #d6d3d1; text-decoration-style: solid; }
main:has(mark.t36:hover) mark.t36 { background: #bfdbfe; text-decoration-style: solid; }
main:has(mark.t37:hover) mark.t37 { background: #fef08a; text-decoration-style: solid; }
main:has(mark.t38:hover) mark.t38 { background: #e9d5ff; text-decoration-style: solid; }
main:has(mark.t39:hover) mark.t39 { background: #fed7aa; text-decoration-style: solid; }
main:has(mark.t40:hover) mark.t40 { background: #99f6e4; text-decoration-style: solid; }
main:has(mark.t41:hover) mark.t41 { background: #fecdd3; text-decoration-style: solid; }
main:has(mark.t42:hover) mark.t42 { background: #bbf7d0; text-decoration-style: solid; }
main:has(mark.t43:hover) mark.t43 { background: #f5d0fe; text-decoration-style: solid; }
main:has(mark.t44:hover) mark.t44 { background: #a5f3fc; text-decoration-style: solid; }
main:has(mark.t45:hover) mark.t45 { background: #d9f99d; text-decoration-style: solid; }
main:has(mark.t46:hover) mark.t46 { background: #c7d2fe; text-decoration-style: solid; }
main:has(mark.t47:hover) mark.t47 { background: #d6d3d1; text-decoration-style: solid; }
.hl-note label { margin-right: 0.75rem; }
main.hl-click pre.prose { color: #9ca3af; }
main.hl-click pre.prose .kw { color: #9ca3af; }
main.hl-click pre.prose mark:not(.hl-pick) { background: none; text-decoration-color: #9ca3af; }
main.hl-click pre.prose mark.hl-pick { color: #111827; }
body:has(input.hl-toggle:not(:checked)) pre.prose mark { background: none; text-decoration: none; color: inherit; }
body:has(input.hl-toggle:not(:checked)) pre.prose .kw { color: inherit; }
@media print {
body { max-width: none; padding: 0; }
nav.crumbs, nav.docnav, .skip, form, .verdict-entry, .hl-note { display: none; }
mark, mark[class] { background: none; text-decoration-color: #4b5563; }
main.hl-click pre.prose, main.hl-click pre.prose mark.hl-pick { color: inherit; }
main.hl-click pre.prose .kw { color: #4b5563; }
main.hl-click pre.prose mark:not(.hl-pick) { text-decoration-color: #4b5563; }
details::details-content { content-visibility: visible; }
pre { border: none; padding: 0; white-space: pre-wrap; overflow-x: visible; }
a { color: inherit; text-decoration: none; }
.chip { border: 1px solid #111827; background: none; color: inherit; }
}"##@
}

pub proof fn css_text_block0(i: int)
    requires
        0 <= i < 64,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(0, 64) == seq![
        98u8,
        111,
        100,
        121,
        32,
        123,
        32,
        109,
        97,
        114,
        103,
        105,
        110,
        58,
        32,
        48,
        32,
        97,
        117,
        116,
        111,
        59,
        32,
        109,
        97,
        120,
        45,
        119,
        105,
        100,
        116,
        104,
        58,
        32,
        55,
        50,
        114,
        101,
        109,
        59,
        32,
        112,
        97,
        100,
        100,
        105,
        110,
        103,
        58,
        32,
        48,
        32,
        49,
        46,
        53,
        114,
        101,
        109,
        32,
        52,
        114,
        101,
        109,
        59,
    ]) by (compute_only);
}

pub proof fn css_text_block1(i: int)
    requires
        64 <= i < 128,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(64, 128) == seq![
        32u8,
        102,
        111,
        110,
        116,
        45,
        102,
        97,
        109,
        105,
        108,
        121,
        58,
        32,
        115,
        121,
        115,
        116,
        101,
        109,
        45,
        117,
        105,
        44,
        32,
        115,
        97,
        110,
        115,
        45,
        115,
        101,
        114,
        105,
        102,
        59,
        32,
        108,
        105,
        110,
        101,
        45,
        104,
        101,
        105,
        103,
        104,
        116,
        58,
        32,
        49,
        46,
        53,
        53,
        59,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
    ]) by (compute_only);
}

pub proof fn css_text_block2(i: int)
    requires
        128 <= i < 192,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(128, 192) == seq![
        49u8,
        49,
        49,
        56,
        50,
        55,
        59,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        102,
        102,
        102,
        102,
        102,
        59,
        32,
        125,
        10,
        97,
        32,
        123,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        49,
        100,
        52,
        101,
        100,
        56,
        59,
        32,
        125,
        10,
        97,
        58,
        102,
        111,
        99,
        117,
        115,
        45,
        118,
        105,
        115,
    ]) by (compute_only);
}

pub proof fn css_text_block3(i: int)
    requires
        192 <= i < 256,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(192, 256) == seq![
        105u8,
        98,
        108,
        101,
        44,
        32,
        115,
        117,
        109,
        109,
        97,
        114,
        121,
        58,
        102,
        111,
        99,
        117,
        115,
        45,
        118,
        105,
        115,
        105,
        98,
        108,
        101,
        32,
        123,
        32,
        111,
        117,
        116,
        108,
        105,
        110,
        101,
        58,
        32,
        51,
        112,
        120,
        32,
        115,
        111,
        108,
        105,
        100,
        32,
        35,
        49,
        100,
        52,
        101,
        100,
        56,
        59,
        32,
        111,
        117,
        116,
        108,
        105,
        110,
    ]) by (compute_only);
}

pub proof fn css_text_block4(i: int)
    requires
        256 <= i < 320,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(256, 320) == seq![
        101u8,
        45,
        111,
        102,
        102,
        115,
        101,
        116,
        58,
        32,
        50,
        112,
        120,
        59,
        32,
        125,
        10,
        46,
        115,
        107,
        105,
        112,
        32,
        123,
        32,
        112,
        111,
        115,
        105,
        116,
        105,
        111,
        110,
        58,
        32,
        97,
        98,
        115,
        111,
        108,
        117,
        116,
        101,
        59,
        32,
        108,
        101,
        102,
        116,
        58,
        32,
        45,
        57,
        57,
        57,
        112,
        120,
        59,
        32,
        116,
        111,
        112,
        58,
        32,
    ]) by (compute_only);
}

pub proof fn css_text_block5(i: int)
    requires
        320 <= i < 384,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(320, 384) == seq![
        48u8,
        59,
        32,
        112,
        97,
        100,
        100,
        105,
        110,
        103,
        58,
        32,
        48,
        46,
        53,
        114,
        101,
        109,
        32,
        49,
        114,
        101,
        109,
        59,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        102,
        102,
        102,
        102,
        102,
        59,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        49,
        100,
        52,
        101,
        100,
        56,
        59,
        32,
        125,
        10,
    ]) by (compute_only);
}

pub proof fn css_text_block6(i: int)
    requires
        384 <= i < 448,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(384, 448) == seq![
        46u8,
        115,
        107,
        105,
        112,
        58,
        102,
        111,
        99,
        117,
        115,
        32,
        123,
        32,
        108,
        101,
        102,
        116,
        58,
        32,
        48,
        59,
        32,
        122,
        45,
        105,
        110,
        100,
        101,
        120,
        58,
        32,
        49,
        59,
        32,
        125,
        10,
        110,
        97,
        118,
        46,
        99,
        114,
        117,
        109,
        98,
        115,
        32,
        123,
        32,
        112,
        97,
        100,
        100,
        105,
        110,
        103,
        58,
        32,
        49,
        114,
        101,
        109,
        32,
    ]) by (compute_only);
}

pub proof fn css_text_block7(i: int)
    requires
        448 <= i < 512,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(448, 512) == seq![
        48u8,
        59,
        32,
        98,
        111,
        114,
        100,
        101,
        114,
        45,
        98,
        111,
        116,
        116,
        111,
        109,
        58,
        32,
        49,
        112,
        120,
        32,
        115,
        111,
        108,
        105,
        100,
        32,
        35,
        101,
        53,
        101,
        55,
        101,
        98,
        59,
        32,
        125,
        10,
        104,
        49,
        32,
        123,
        32,
        102,
        111,
        110,
        116,
        45,
        115,
        105,
        122,
        101,
        58,
        32,
        49,
        46,
        53,
        114,
        101,
        109,
        59,
        32,
        125,
    ]) by (compute_only);
}

pub proof fn css_text_block8(i: int)
    requires
        512 <= i < 576,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(512, 576) == seq![
        10u8,
        104,
        50,
        32,
        123,
        32,
        102,
        111,
        110,
        116,
        45,
        115,
        105,
        122,
        101,
        58,
        32,
        49,
        46,
        50,
        53,
        114,
        101,
        109,
        59,
        32,
        125,
        10,
        104,
        51,
        32,
        123,
        32,
        102,
        111,
        110,
        116,
        45,
        115,
        105,
        122,
        101,
        58,
        32,
        49,
        46,
        48,
        53,
        114,
        101,
        109,
        59,
        32,
        125,
        10,
        104,
        49,
        32,
        97,
        46,
        115,
        111,
        117,
        114,
    ]) by (compute_only);
}

pub proof fn css_text_block9(i: int)
    requires
        576 <= i < 640,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(576, 640) == seq![
        99u8,
        101,
        32,
        123,
        32,
        102,
        111,
        110,
        116,
        45,
        115,
        105,
        122,
        101,
        58,
        32,
        49,
        114,
        101,
        109,
        59,
        32,
        102,
        111,
        110,
        116,
        45,
        119,
        101,
        105,
        103,
        104,
        116,
        58,
        32,
        52,
        48,
        48,
        59,
        32,
        109,
        97,
        114,
        103,
        105,
        110,
        45,
        108,
        101,
        102,
        116,
        58,
        32,
        48,
        46,
        53,
        114,
        101,
        109,
        59,
        32,
        125,
        10,
        116,
    ]) by (compute_only);
}

pub proof fn css_text_block10(i: int)
    requires
        640 <= i < 704,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(640, 704) == seq![
        97u8,
        98,
        108,
        101,
        32,
        123,
        32,
        98,
        111,
        114,
        100,
        101,
        114,
        45,
        99,
        111,
        108,
        108,
        97,
        112,
        115,
        101,
        58,
        32,
        99,
        111,
        108,
        108,
        97,
        112,
        115,
        101,
        59,
        32,
        119,
        105,
        100,
        116,
        104,
        58,
        32,
        49,
        48,
        48,
        37,
        59,
        32,
        109,
        97,
        114,
        103,
        105,
        110,
        58,
        32,
        49,
        114,
        101,
        109,
        32,
        48,
        59,
        32,
        125,
    ]) by (compute_only);
}

pub proof fn css_text_block11(i: int)
    requires
        704 <= i < 768,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(704, 768) == seq![
        10u8,
        116,
        104,
        44,
        32,
        116,
        100,
        32,
        123,
        32,
        116,
        101,
        120,
        116,
        45,
        97,
        108,
        105,
        103,
        110,
        58,
        32,
        108,
        101,
        102,
        116,
        59,
        32,
        112,
        97,
        100,
        100,
        105,
        110,
        103,
        58,
        32,
        48,
        46,
        52,
        114,
        101,
        109,
        32,
        48,
        46,
        54,
        114,
        101,
        109,
        59,
        32,
        98,
        111,
        114,
        100,
        101,
        114,
        45,
        98,
        111,
        116,
        116,
        111,
    ]) by (compute_only);
}

pub proof fn css_text_block12(i: int)
    requires
        768 <= i < 832,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(768, 832) == seq![
        109u8,
        58,
        32,
        49,
        112,
        120,
        32,
        115,
        111,
        108,
        105,
        100,
        32,
        35,
        101,
        53,
        101,
        55,
        101,
        98,
        59,
        32,
        118,
        101,
        114,
        116,
        105,
        99,
        97,
        108,
        45,
        97,
        108,
        105,
        103,
        110,
        58,
        32,
        116,
        111,
        112,
        59,
        32,
        125,
        10,
        116,
        104,
        32,
        123,
        32,
        98,
        111,
        114,
        100,
        101,
        114,
        45,
        98,
        111,
        116,
        116,
        111,
        109,
        58,
    ]) by (compute_only);
}

pub proof fn css_text_block13(i: int)
    requires
        832 <= i < 896,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(832, 896) == seq![
        32u8,
        50,
        112,
        120,
        32,
        115,
        111,
        108,
        105,
        100,
        32,
        35,
        49,
        49,
        49,
        56,
        50,
        55,
        59,
        32,
        125,
        10,
        116,
        97,
        98,
        108,
        101,
        46,
        99,
        111,
        109,
        112,
        97,
        99,
        116,
        32,
        123,
        32,
        119,
        105,
        100,
        116,
        104,
        58,
        32,
        97,
        117,
        116,
        111,
        59,
        32,
        125,
        10,
        116,
        97,
        98,
        108,
        101,
        46,
        99,
        111,
        109,
        112,
        97,
    ]) by (compute_only);
}

pub proof fn css_text_block14(i: int)
    requires
        896 <= i < 960,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(896, 960) == seq![
        99u8,
        116,
        32,
        116,
        104,
        44,
        32,
        116,
        97,
        98,
        108,
        101,
        46,
        99,
        111,
        109,
        112,
        97,
        99,
        116,
        32,
        116,
        100,
        32,
        123,
        32,
        112,
        97,
        100,
        100,
        105,
        110,
        103,
        45,
        114,
        105,
        103,
        104,
        116,
        58,
        32,
        50,
        114,
        101,
        109,
        59,
        32,
        125,
        10,
        116,
        97,
        98,
        108,
        101,
        46,
        114,
        101,
        99,
        111,
        114,
        100,
        115,
        32,
        123,
    ]) by (compute_only);
}

pub proof fn css_text_block15(i: int)
    requires
        960 <= i < 1024,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(960, 1024) == seq![
        32u8,
        116,
        97,
        98,
        108,
        101,
        45,
        108,
        97,
        121,
        111,
        117,
        116,
        58,
        32,
        102,
        105,
        120,
        101,
        100,
        59,
        32,
        125,
        10,
        116,
        97,
        98,
        108,
        101,
        46,
        114,
        101,
        99,
        111,
        114,
        100,
        115,
        32,
        116,
        104,
        32,
        123,
        32,
        98,
        111,
        120,
        45,
        115,
        105,
        122,
        105,
        110,
        103,
        58,
        32,
        98,
        111,
        114,
        100,
        101,
        114,
        45,
        98,
        111,
    ]) by (compute_only);
}

pub proof fn css_text_block16(i: int)
    requires
        1024 <= i < 1088,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1024, 1088) == seq![
        120u8,
        59,
        32,
        125,
        10,
        116,
        97,
        98,
        108,
        101,
        46,
        114,
        101,
        99,
        111,
        114,
        100,
        115,
        32,
        116,
        104,
        58,
        110,
        116,
        104,
        45,
        99,
        104,
        105,
        108,
        100,
        40,
        49,
        41,
        32,
        123,
        32,
        119,
        105,
        100,
        116,
        104,
        58,
        32,
        49,
        50,
        37,
        59,
        32,
        125,
        10,
        116,
        97,
        98,
        108,
        101,
        46,
        114,
        101,
        99,
        111,
        114,
        100,
        115,
    ]) by (compute_only);
}

pub proof fn css_text_block17(i: int)
    requires
        1088 <= i < 1152,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1088, 1152) == seq![
        32u8,
        116,
        104,
        58,
        110,
        116,
        104,
        45,
        99,
        104,
        105,
        108,
        100,
        40,
        50,
        41,
        32,
        123,
        32,
        119,
        105,
        100,
        116,
        104,
        58,
        32,
        49,
        56,
        37,
        59,
        32,
        125,
        10,
        116,
        97,
        98,
        108,
        101,
        46,
        114,
        101,
        99,
        111,
        114,
        100,
        115,
        32,
        116,
        104,
        58,
        110,
        116,
        104,
        45,
        99,
        104,
        105,
        108,
        100,
        40,
        51,
        41,
        32,
        123,
    ]) by (compute_only);
}

pub proof fn css_text_block18(i: int)
    requires
        1152 <= i < 1216,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1152, 1216) == seq![
        32u8,
        119,
        105,
        100,
        116,
        104,
        58,
        32,
        50,
        50,
        37,
        59,
        32,
        125,
        10,
        116,
        97,
        98,
        108,
        101,
        46,
        114,
        101,
        99,
        111,
        114,
        100,
        115,
        32,
        116,
        104,
        58,
        110,
        116,
        104,
        45,
        99,
        104,
        105,
        108,
        100,
        40,
        52,
        41,
        32,
        123,
        32,
        119,
        105,
        100,
        116,
        104,
        58,
        32,
        49,
        48,
        37,
        59,
        32,
        125,
        10,
        46,
        99,
        104,
    ]) by (compute_only);
}

pub proof fn css_text_block19(i: int)
    requires
        1216 <= i < 1280,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1216, 1280) == seq![
        105u8,
        112,
        32,
        123,
        32,
        100,
        105,
        115,
        112,
        108,
        97,
        121,
        58,
        32,
        105,
        110,
        108,
        105,
        110,
        101,
        45,
        98,
        108,
        111,
        99,
        107,
        59,
        32,
        112,
        97,
        100,
        100,
        105,
        110,
        103,
        58,
        32,
        48,
        46,
        49,
        114,
        101,
        109,
        32,
        48,
        46,
        54,
        114,
        101,
        109,
        59,
        32,
        98,
        111,
        114,
        100,
        101,
        114,
        45,
        114,
        97,
        100,
        105,
        117,
    ]) by (compute_only);
}

pub proof fn css_text_block20(i: int)
    requires
        1280 <= i < 1344,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1280, 1344) == seq![
        115u8,
        58,
        32,
        57,
        57,
        57,
        112,
        120,
        59,
        32,
        102,
        111,
        110,
        116,
        45,
        115,
        105,
        122,
        101,
        58,
        32,
        48,
        46,
        56,
        53,
        114,
        101,
        109,
        59,
        32,
        102,
        111,
        110,
        116,
        45,
        119,
        101,
        105,
        103,
        104,
        116,
        58,
        32,
        54,
        48,
        48,
        59,
        32,
        125,
        10,
        46,
        99,
        104,
        105,
        112,
        45,
        97,
        112,
        112,
        114,
        111,
        118,
        101,
        100,
    ]) by (compute_only);
}

pub proof fn css_text_block21(i: int)
    requires
        1344 <= i < 1408,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1344, 1408) == seq![
        32u8,
        123,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        49,
        52,
        53,
        51,
        50,
        100,
        59,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        100,
        99,
        102,
        99,
        101,
        55,
        59,
        32,
        125,
        10,
        46,
        99,
        104,
        105,
        112,
        45,
        114,
        101,
        106,
        101,
        99,
        116,
        101,
        100,
        32,
        123,
        32,
        99,
        111,
        108,
        111,
        114,
    ]) by (compute_only);
}

pub proof fn css_text_block22(i: int)
    requires
        1408 <= i < 1472,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1408, 1472) == seq![
        58u8,
        32,
        35,
        55,
        102,
        49,
        100,
        49,
        100,
        59,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        101,
        101,
        50,
        101,
        50,
        59,
        32,
        125,
        10,
        46,
        99,
        104,
        105,
        112,
        45,
        99,
        111,
        110,
        116,
        101,
        115,
        116,
        101,
        100,
        32,
        123,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        52,
        99,
        49,
        100,
    ]) by (compute_only);
}

pub proof fn css_text_block23(i: int)
    requires
        1472 <= i < 1536,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1472, 1536) == seq![
        57u8,
        53,
        59,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        101,
        100,
        101,
        57,
        102,
        101,
        59,
        32,
        125,
        10,
        46,
        99,
        104,
        105,
        112,
        45,
        115,
        116,
        97,
        108,
        101,
        32,
        123,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        55,
        56,
        51,
        53,
        48,
        102,
        59,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
    ]) by (compute_only);
}

pub proof fn css_text_block24(i: int)
    requires
        1536 <= i < 1600,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1536, 1600) == seq![
        117u8,
        110,
        100,
        58,
        32,
        35,
        102,
        101,
        102,
        51,
        99,
        55,
        59,
        32,
        125,
        10,
        46,
        99,
        104,
        105,
        112,
        45,
        117,
        110,
        114,
        101,
        118,
        105,
        101,
        119,
        101,
        100,
        32,
        123,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        49,
        102,
        50,
        57,
        51,
        55,
        59,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
    ]) by (compute_only);
}

pub proof fn css_text_block25(i: int)
    requires
        1600 <= i < 1664,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1600, 1664) == seq![
        101u8,
        53,
        101,
        55,
        101,
        98,
        59,
        32,
        125,
        10,
        112,
        114,
        101,
        32,
        123,
        32,
        112,
        97,
        100,
        100,
        105,
        110,
        103,
        58,
        32,
        48,
        46,
        55,
        53,
        114,
        101,
        109,
        32,
        49,
        114,
        101,
        109,
        59,
        32,
        98,
        111,
        114,
        100,
        101,
        114,
        58,
        32,
        49,
        112,
        120,
        32,
        115,
        111,
        108,
        105,
        100,
        32,
        35,
        101,
        53,
        101,
        55,
        101,
        98,
    ]) by (compute_only);
}

pub proof fn css_text_block26(i: int)
    requires
        1664 <= i < 1728,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1664, 1728) == seq![
        59u8,
        32,
        119,
        104,
        105,
        116,
        101,
        45,
        115,
        112,
        97,
        99,
        101,
        58,
        32,
        112,
        114,
        101,
        45,
        119,
        114,
        97,
        112,
        59,
        32,
        111,
        118,
        101,
        114,
        102,
        108,
        111,
        119,
        45,
        120,
        58,
        32,
        97,
        117,
        116,
        111,
        59,
        32,
        125,
        10,
        112,
        114,
        101,
        44,
        32,
        99,
        111,
        100,
        101,
        32,
        123,
        32,
        102,
        111,
        110,
        116,
        45,
        102,
        97,
    ]) by (compute_only);
}

pub proof fn css_text_block27(i: int)
    requires
        1728 <= i < 1792,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1728, 1792) == seq![
        109u8,
        105,
        108,
        121,
        58,
        32,
        117,
        105,
        45,
        109,
        111,
        110,
        111,
        115,
        112,
        97,
        99,
        101,
        44,
        32,
        77,
        101,
        110,
        108,
        111,
        44,
        32,
        67,
        111,
        110,
        115,
        111,
        108,
        97,
        115,
        44,
        32,
        109,
        111,
        110,
        111,
        115,
        112,
        97,
        99,
        101,
        59,
        32,
        102,
        111,
        110,
        116,
        45,
        115,
        105,
        122,
        101,
        58,
        32,
        48,
        46,
        57,
        53,
        114,
    ]) by (compute_only);
}

pub proof fn css_text_block28(i: int)
    requires
        1792 <= i < 1856,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1792, 1856) == seq![
        101u8,
        109,
        59,
        32,
        125,
        10,
        112,
        114,
        101,
        46,
        112,
        114,
        111,
        115,
        101,
        32,
        123,
        32,
        102,
        111,
        110,
        116,
        45,
        102,
        97,
        109,
        105,
        108,
        121,
        58,
        32,
        71,
        101,
        111,
        114,
        103,
        105,
        97,
        44,
        32,
        115,
        101,
        114,
        105,
        102,
        59,
        32,
        102,
        111,
        110,
        116,
        45,
        115,
        105,
        122,
        101,
        58,
        32,
        49,
        46,
        48,
        53,
        114,
        101,
    ]) by (compute_only);
}

pub proof fn css_text_block29(i: int)
    requires
        1856 <= i < 1920,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1856, 1920) == seq![
        109u8,
        59,
        32,
        111,
        118,
        101,
        114,
        102,
        108,
        111,
        119,
        45,
        119,
        114,
        97,
        112,
        58,
        32,
        97,
        110,
        121,
        119,
        104,
        101,
        114,
        101,
        59,
        32,
        125,
        10,
        100,
        116,
        32,
        123,
        32,
        102,
        111,
        110,
        116,
        45,
        119,
        101,
        105,
        103,
        104,
        116,
        58,
        32,
        54,
        48,
        48,
        59,
        32,
        109,
        97,
        114,
        103,
        105,
        110,
        45,
        116,
        111,
        112,
        58,
    ]) by (compute_only);
}

pub proof fn css_text_block30(i: int)
    requires
        1920 <= i < 1984,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1920, 1984) == seq![
        32u8,
        48,
        46,
        54,
        114,
        101,
        109,
        59,
        32,
        125,
        10,
        100,
        100,
        32,
        123,
        32,
        109,
        97,
        114,
        103,
        105,
        110,
        45,
        108,
        101,
        102,
        116,
        58,
        32,
        48,
        59,
        32,
        125,
        10,
        115,
        117,
        109,
        109,
        97,
        114,
        121,
        32,
        123,
        32,
        99,
        117,
        114,
        115,
        111,
        114,
        58,
        32,
        112,
        111,
        105,
        110,
        116,
        101,
        114,
        59,
        32,
        125,
        10,
        115,
    ]) by (compute_only);
}

pub proof fn css_text_block31(i: int)
    requires
        1984 <= i < 2048,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(1984, 2048) == seq![
        101u8,
        99,
        116,
        105,
        111,
        110,
        32,
        123,
        32,
        109,
        97,
        114,
        103,
        105,
        110,
        58,
        32,
        49,
        46,
        53,
        114,
        101,
        109,
        32,
        48,
        59,
        32,
        125,
        10,
        110,
        97,
        118,
        46,
        100,
        111,
        99,
        110,
        97,
        118,
        32,
        123,
        32,
        112,
        97,
        100,
        100,
        105,
        110,
        103,
        58,
        32,
        49,
        114,
        101,
        109,
        32,
        48,
        59,
        32,
        98,
        111,
        114,
        100,
        101,
    ]) by (compute_only);
}

pub proof fn css_text_block32(i: int)
    requires
        2048 <= i < 2112,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(2048, 2112) == seq![
        114u8,
        45,
        116,
        111,
        112,
        58,
        32,
        49,
        112,
        120,
        32,
        115,
        111,
        108,
        105,
        100,
        32,
        35,
        101,
        53,
        101,
        55,
        101,
        98,
        59,
        32,
        125,
        10,
        102,
        111,
        111,
        116,
        101,
        114,
        46,
        115,
        99,
        111,
        112,
        101,
        32,
        123,
        32,
        109,
        97,
        114,
        103,
        105,
        110,
        45,
        116,
        111,
        112,
        58,
        32,
        50,
        114,
        101,
        109,
        59,
        32,
        112,
        97,
        100,
    ]) by (compute_only);
}

pub proof fn css_text_block33(i: int)
    requires
        2112 <= i < 2176,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(2112, 2176) == seq![
        100u8,
        105,
        110,
        103,
        58,
        32,
        49,
        114,
        101,
        109,
        32,
        48,
        59,
        32,
        98,
        111,
        114,
        100,
        101,
        114,
        45,
        116,
        111,
        112,
        58,
        32,
        49,
        112,
        120,
        32,
        115,
        111,
        108,
        105,
        100,
        32,
        35,
        101,
        53,
        101,
        55,
        101,
        98,
        59,
        32,
        102,
        111,
        110,
        116,
        45,
        115,
        105,
        122,
        101,
        58,
        32,
        48,
        46,
        57,
        114,
        101,
        109,
        59,
        32,
    ]) by (compute_only);
}

pub proof fn css_text_block34(i: int)
    requires
        2176 <= i < 2240,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(2176, 2240) == seq![
        125u8,
        10,
        102,
        111,
        114,
        109,
        32,
        108,
        97,
        98,
        101,
        108,
        32,
        123,
        32,
        100,
        105,
        115,
        112,
        108,
        97,
        121,
        58,
        32,
        98,
        108,
        111,
        99,
        107,
        59,
        32,
        109,
        97,
        114,
        103,
        105,
        110,
        45,
        116,
        111,
        112,
        58,
        32,
        49,
        114,
        101,
        109,
        59,
        32,
        102,
        111,
        110,
        116,
        45,
        119,
        101,
        105,
        103,
        104,
        116,
        58,
        32,
        54,
        48,
    ]) by (compute_only);
}

pub proof fn css_text_block35(i: int)
    requires
        2240 <= i < 2304,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(2240, 2304) == seq![
        48u8,
        59,
        32,
        125,
        10,
        102,
        105,
        101,
        108,
        100,
        115,
        101,
        116,
        32,
        123,
        32,
        98,
        111,
        114,
        100,
        101,
        114,
        58,
        32,
        48,
        59,
        32,
        109,
        97,
        114,
        103,
        105,
        110,
        58,
        32,
        49,
        114,
        101,
        109,
        32,
        48,
        32,
        48,
        59,
        32,
        112,
        97,
        100,
        100,
        105,
        110,
        103,
        58,
        32,
        48,
        59,
        32,
        109,
        97,
        120,
        45,
        119,
        105,
        100,
    ]) by (compute_only);
}

pub proof fn css_text_block36(i: int)
    requires
        2304 <= i < 2368,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(2304, 2368) == seq![
        116u8,
        104,
        58,
        32,
        50,
        56,
        114,
        101,
        109,
        59,
        32,
        125,
        10,
        108,
        101,
        103,
        101,
        110,
        100,
        32,
        123,
        32,
        102,
        111,
        110,
        116,
        45,
        119,
        101,
        105,
        103,
        104,
        116,
        58,
        32,
        54,
        48,
        48,
        59,
        32,
        112,
        97,
        100,
        100,
        105,
        110,
        103,
        58,
        32,
        48,
        59,
        32,
        125,
        10,
        102,
        105,
        101,
        108,
        100,
        115,
        101,
        116,
        32,
        108,
    ]) by (compute_only);
}

pub proof fn css_text_block37(i: int)
    requires
        2368 <= i < 2432,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(2368, 2432) == seq![
        97u8,
        98,
        101,
        108,
        32,
        123,
        32,
        109,
        97,
        114,
        103,
        105,
        110,
        45,
        116,
        111,
        112,
        58,
        32,
        48,
        46,
        53,
        114,
        101,
        109,
        59,
        32,
        102,
        111,
        110,
        116,
        45,
        119,
        101,
        105,
        103,
        104,
        116,
        58,
        32,
        52,
        48,
        48,
        59,
        32,
        125,
        10,
        105,
        110,
        112,
        117,
        116,
        91,
        116,
        121,
        112,
        101,
        61,
        34,
        116,
        101,
        120,
        116,
        34,
    ]) by (compute_only);
}

pub proof fn css_text_block38(i: int)
    requires
        2432 <= i < 2496,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(2432, 2496) == seq![
        93u8,
        44,
        32,
        116,
        101,
        120,
        116,
        97,
        114,
        101,
        97,
        32,
        123,
        32,
        100,
        105,
        115,
        112,
        108,
        97,
        121,
        58,
        32,
        98,
        108,
        111,
        99,
        107,
        59,
        32,
        98,
        111,
        120,
        45,
        115,
        105,
        122,
        105,
        110,
        103,
        58,
        32,
        98,
        111,
        114,
        100,
        101,
        114,
        45,
        98,
        111,
        120,
        59,
        32,
        119,
        105,
        100,
        116,
        104,
        58,
        32,
        49,
        48,
        48,
    ]) by (compute_only);
}

pub proof fn css_text_block39(i: int)
    requires
        2496 <= i < 2560,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(2496, 2560) == seq![
        37u8,
        59,
        32,
        109,
        97,
        120,
        45,
        119,
        105,
        100,
        116,
        104,
        58,
        32,
        50,
        56,
        114,
        101,
        109,
        59,
        32,
        109,
        97,
        114,
        103,
        105,
        110,
        45,
        116,
        111,
        112,
        58,
        32,
        48,
        46,
        51,
        114,
        101,
        109,
        59,
        32,
        112,
        97,
        100,
        100,
        105,
        110,
        103,
        58,
        32,
        48,
        46,
        52,
        53,
        114,
        101,
        109,
        32,
        48,
        46,
        54,
        114,
        101,
        109,
    ]) by (compute_only);
}

pub proof fn css_text_block40(i: int)
    requires
        2560 <= i < 2624,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(2560, 2624) == seq![
        59u8,
        32,
        98,
        111,
        114,
        100,
        101,
        114,
        58,
        32,
        49,
        112,
        120,
        32,
        115,
        111,
        108,
        105,
        100,
        32,
        35,
        101,
        53,
        101,
        55,
        101,
        98,
        59,
        32,
        102,
        111,
        110,
        116,
        45,
        102,
        97,
        109,
        105,
        108,
        121,
        58,
        32,
        105,
        110,
        104,
        101,
        114,
        105,
        116,
        59,
        32,
        102,
        111,
        110,
        116,
        45,
        115,
        105,
        122,
        101,
        58,
        32,
        49,
        114,
    ]) by (compute_only);
}

pub proof fn css_text_block41(i: int)
    requires
        2624 <= i < 2688,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(2624, 2688) == seq![
        101u8,
        109,
        59,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        49,
        49,
        49,
        56,
        50,
        55,
        59,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        102,
        102,
        102,
        102,
        102,
        59,
        32,
        125,
        10,
        116,
        101,
        120,
        116,
        97,
        114,
        101,
        97,
        32,
        123,
        32,
        109,
        105,
        110,
        45,
        104,
        101,
        105,
        103,
        104,
        116,
    ]) by (compute_only);
}

pub proof fn css_text_block42(i: int)
    requires
        2688 <= i < 2752,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(2688, 2752) == seq![
        58u8,
        32,
        54,
        114,
        101,
        109,
        59,
        32,
        125,
        10,
        105,
        110,
        112,
        117,
        116,
        91,
        116,
        121,
        112,
        101,
        61,
        34,
        114,
        97,
        100,
        105,
        111,
        34,
        93,
        44,
        32,
        105,
        110,
        112,
        117,
        116,
        91,
        116,
        121,
        112,
        101,
        61,
        34,
        99,
        104,
        101,
        99,
        107,
        98,
        111,
        120,
        34,
        93,
        32,
        123,
        32,
        97,
        99,
        99,
        101,
        110,
        116,
        45,
        99,
    ]) by (compute_only);
}

pub proof fn css_text_block43(i: int)
    requires
        2752 <= i < 2816,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(2752, 2816) == seq![
        111u8,
        108,
        111,
        114,
        58,
        32,
        35,
        49,
        49,
        49,
        56,
        50,
        55,
        59,
        32,
        125,
        10,
        105,
        110,
        112,
        117,
        116,
        91,
        116,
        121,
        112,
        101,
        61,
        34,
        116,
        101,
        120,
        116,
        34,
        93,
        58,
        102,
        111,
        99,
        117,
        115,
        45,
        118,
        105,
        115,
        105,
        98,
        108,
        101,
        44,
        32,
        105,
        110,
        112,
        117,
        116,
        91,
        116,
        121,
        112,
        101,
        61,
        34,
        114,
    ]) by (compute_only);
}

pub proof fn css_text_block44(i: int)
    requires
        2816 <= i < 2880,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(2816, 2880) == seq![
        97u8,
        100,
        105,
        111,
        34,
        93,
        58,
        102,
        111,
        99,
        117,
        115,
        45,
        118,
        105,
        115,
        105,
        98,
        108,
        101,
        44,
        32,
        105,
        110,
        112,
        117,
        116,
        91,
        116,
        121,
        112,
        101,
        61,
        34,
        99,
        104,
        101,
        99,
        107,
        98,
        111,
        120,
        34,
        93,
        58,
        102,
        111,
        99,
        117,
        115,
        45,
        118,
        105,
        115,
        105,
        98,
        108,
        101,
        44,
        32,
        116,
        101,
        120,
        116,
    ]) by (compute_only);
}

pub proof fn css_text_block45(i: int)
    requires
        2880 <= i < 2944,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(2880, 2944) == seq![
        97u8,
        114,
        101,
        97,
        58,
        102,
        111,
        99,
        117,
        115,
        45,
        118,
        105,
        115,
        105,
        98,
        108,
        101,
        44,
        32,
        98,
        117,
        116,
        116,
        111,
        110,
        58,
        102,
        111,
        99,
        117,
        115,
        45,
        118,
        105,
        115,
        105,
        98,
        108,
        101,
        32,
        123,
        32,
        111,
        117,
        116,
        108,
        105,
        110,
        101,
        58,
        32,
        51,
        112,
        120,
        32,
        115,
        111,
        108,
        105,
        100,
        32,
        35,
        49,
    ]) by (compute_only);
}

pub proof fn css_text_block46(i: int)
    requires
        2944 <= i < 3008,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(2944, 3008) == seq![
        100u8,
        52,
        101,
        100,
        56,
        59,
        32,
        111,
        117,
        116,
        108,
        105,
        110,
        101,
        45,
        111,
        102,
        102,
        115,
        101,
        116,
        58,
        32,
        50,
        112,
        120,
        59,
        32,
        125,
        10,
        98,
        117,
        116,
        116,
        111,
        110,
        32,
        123,
        32,
        109,
        97,
        114,
        103,
        105,
        110,
        45,
        116,
        111,
        112,
        58,
        32,
        49,
        46,
        50,
        53,
        114,
        101,
        109,
        59,
        32,
        112,
        97,
        100,
        100,
    ]) by (compute_only);
}

pub proof fn css_text_block47(i: int)
    requires
        3008 <= i < 3072,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3008, 3072) == seq![
        105u8,
        110,
        103,
        58,
        32,
        48,
        46,
        53,
        114,
        101,
        109,
        32,
        49,
        46,
        50,
        114,
        101,
        109,
        59,
        32,
        98,
        111,
        114,
        100,
        101,
        114,
        58,
        32,
        49,
        112,
        120,
        32,
        115,
        111,
        108,
        105,
        100,
        32,
        35,
        49,
        49,
        49,
        56,
        50,
        55,
        59,
        32,
        102,
        111,
        110,
        116,
        45,
        102,
        97,
        109,
        105,
        108,
        121,
        58,
        32,
        105,
        110,
        104,
        101,
    ]) by (compute_only);
}

pub proof fn css_text_block48(i: int)
    requires
        3072 <= i < 3136,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3072, 3136) == seq![
        114u8,
        105,
        116,
        59,
        32,
        102,
        111,
        110,
        116,
        45,
        115,
        105,
        122,
        101,
        58,
        32,
        49,
        114,
        101,
        109,
        59,
        32,
        102,
        111,
        110,
        116,
        45,
        119,
        101,
        105,
        103,
        104,
        116,
        58,
        32,
        54,
        48,
        48,
        59,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        102,
        102,
        102,
        102,
        102,
        102,
        59,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
    ]) by (compute_only);
}

pub proof fn css_text_block49(i: int)
    requires
        3136 <= i < 3200,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3136, 3200) == seq![
        110u8,
        100,
        58,
        32,
        35,
        49,
        49,
        49,
        56,
        50,
        55,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        100,
        98,
        101,
        97,
        102,
        101,
        59,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        105,
        110,
        104,
        101,
        114,
        105,
        116,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
    ]) by (compute_only);
}

pub proof fn css_text_block50(i: int)
    requires
        3200 <= i < 3264,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3200, 3264) == seq![
        100u8,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        58,
        32,
        117,
        110,
        100,
        101,
        114,
        108,
        105,
        110,
        101,
        32,
        100,
        111,
        116,
        116,
        101,
        100,
        32,
        35,
        52,
        98,
        53,
        53,
        54,
        51,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        117,
        110,
        100,
        101,
        114,
        108,
        105,
        110,
        101,
        45,
        111,
        102,
        102,
        115,
        101,
        116,
        58,
        32,
        48,
        46,
        49,
    ]) by (compute_only);
}

pub proof fn css_text_block51(i: int)
    requires
        3264 <= i < 3328,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3264, 3328) == seq![
        53u8,
        101,
        109,
        59,
        32,
        125,
        10,
        46,
        107,
        119,
        32,
        123,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        52,
        98,
        53,
        53,
        54,
        51,
        59,
        32,
        125,
        10,
        46,
        104,
        108,
        45,
        110,
        111,
        116,
        101,
        32,
        123,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        52,
        98,
        53,
        53,
        54,
        51,
        59,
        32,
        102,
        111,
        110,
        116,
        45,
        115,
    ]) by (compute_only);
}

pub proof fn css_text_block52(i: int)
    requires
        3328 <= i < 3392,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3328, 3392) == seq![
        105u8,
        122,
        101,
        58,
        32,
        48,
        46,
        57,
        114,
        101,
        109,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        51,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        53,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        55,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
    ]) by (compute_only);
}

pub proof fn css_text_block53(i: int)
    requires
        3392 <= i < 3456,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3392, 3456) == seq![
        100u8,
        58,
        32,
        35,
        102,
        101,
        102,
        57,
        99,
        51,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        52,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        54,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        56,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
    ]) by (compute_only);
}

pub proof fn css_text_block54(i: int)
    requires
        3456 <= i < 3520,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3456, 3520) == seq![
        58u8,
        32,
        35,
        102,
        51,
        101,
        56,
        102,
        102,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        53,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        55,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        57,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
    ]) by (compute_only);
}

pub proof fn css_text_block55(i: int)
    requires
        3520 <= i < 3584,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3520, 3584) == seq![
        32u8,
        35,
        102,
        102,
        101,
        100,
        100,
        53,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        54,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        56,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        48,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
    ]) by (compute_only);
}

pub proof fn css_text_block56(i: int)
    requires
        3584 <= i < 3648,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3584, 3648) == seq![
        35u8,
        99,
        99,
        102,
        98,
        102,
        49,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        53,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        55,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        57,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        49,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
    ]) by (compute_only);
}

pub proof fn css_text_block57(i: int)
    requires
        3648 <= i < 3712,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3648, 3712) == seq![
        102u8,
        102,
        101,
        52,
        101,
        54,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        54,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        56,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        48,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        50,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        100,
    ]) by (compute_only);
}

pub proof fn css_text_block58(i: int)
    requires
        3712 <= i < 3776,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3712, 3776) == seq![
        99u8,
        102,
        99,
        101,
        55,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        55,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        57,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        49,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        51,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        97,
    ]) by (compute_only);
}

pub proof fn css_text_block59(i: int)
    requires
        3776 <= i < 3840,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3776, 3840) == seq![
        101u8,
        56,
        102,
        102,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        56,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        48,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        50,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        52,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        99,
        102,
        102,
    ]) by (compute_only);
}

pub proof fn css_text_block60(i: int)
    requires
        3840 <= i < 3904,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3840, 3904) == seq![
        97u8,
        102,
        101,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        57,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        49,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        51,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        53,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        101,
        99,
        102,
        99,
    ]) by (compute_only);
}

pub proof fn css_text_block61(i: int)
    requires
        3904 <= i < 3968,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3904, 3968) == seq![
        99u8,
        98,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        48,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        50,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        52,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        54,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        101,
        48,
        101,
        55,
    ]) by (compute_only);
}

pub proof fn css_text_block62(i: int)
    requires
        3968 <= i < 4032,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(3968, 4032) == seq![
        102u8,
        102,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        49,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        51,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        53,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        55,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        101,
        55,
        101,
        53,
    ]) by (compute_only);
}

pub proof fn css_text_block63(i: int)
    requires
        4032 <= i < 4096,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4032, 4096) == seq![
        101u8,
        52,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        48,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        50,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        52,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        54,
        32,
        123,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        99,
        111,
    ]) by (compute_only);
}

pub proof fn css_text_block64(i: int)
    requires
        4096 <= i < 4160,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4096, 4160) == seq![
        108u8,
        111,
        114,
        58,
        32,
        35,
        50,
        53,
        54,
        51,
        101,
        98,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        51,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        53,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        55,
        32,
        123,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
    ]) by (compute_only);
}

pub proof fn css_text_block65(i: int)
    requires
        4160 <= i < 4224,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4160, 4224) == seq![
        111u8,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        97,
        49,
        54,
        50,
        48,
        55,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        52,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        54,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        56,
        32,
    ]) by (compute_only);
}

pub proof fn css_text_block66(i: int)
    requires
        4224 <= i < 4288,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4224, 4288) == seq![
        123u8,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        55,
        99,
        51,
        97,
        101,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        53,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        55,
        44,
    ]) by (compute_only);
}

pub proof fn css_text_block67(i: int)
    requires
        4288 <= i < 4352,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4288, 4352) == seq![
        32u8,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        57,
        32,
        123,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        99,
        50,
        52,
        49,
        48,
        99,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        54,
        44,
    ]) by (compute_only);
}

pub proof fn css_text_block68(i: int)
    requires
        4352 <= i < 4416,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4352, 4416) == seq![
        32u8,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        56,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        48,
        32,
        123,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        48,
        102,
        55,
        54,
        54,
        101,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        53,
        44,
    ]) by (compute_only);
}

pub proof fn css_text_block69(i: int)
    requires
        4416 <= i < 4480,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4416, 4480) == seq![
        32u8,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        55,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        57,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        49,
        32,
        123,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        98,
        101,
        49,
        50,
        51,
        99,
        59,
        32,
    ]) by (compute_only);
}

pub proof fn css_text_block70(i: int)
    requires
        4480 <= i < 4544,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4480, 4544) == seq![
        125u8,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        54,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        56,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        48,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        50,
        32,
        123,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        99,
        111,
        108,
        111,
        114,
        58,
    ]) by (compute_only);
}

pub proof fn css_text_block71(i: int)
    requires
        4544 <= i < 4608,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4544, 4608) == seq![
        32u8,
        35,
        49,
        53,
        56,
        48,
        51,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        55,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        57,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        49,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        51,
        32,
        123,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
    ]) by (compute_only);
}

pub proof fn css_text_block72(i: int)
    requires
        4608 <= i < 4672,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4608, 4672) == seq![
        105u8,
        111,
        110,
        45,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        97,
        50,
        49,
        99,
        97,
        102,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        56,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        48,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        50,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        52,
        32,
        123,
        32,
        116,
        101,
    ]) by (compute_only);
}

pub proof fn css_text_block73(i: int)
    requires
        4672 <= i < 4736,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4672, 4736) == seq![
        120u8,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        48,
        101,
        55,
        52,
        57,
        48,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        57,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        49,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        51,
        44,
        32,
        109,
        97,
        114,
    ]) by (compute_only);
}

pub proof fn css_text_block74(i: int)
    requires
        4736 <= i < 4800,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4736, 4800) == seq![
        107u8,
        46,
        116,
        52,
        53,
        32,
        123,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        52,
        100,
        55,
        99,
        48,
        102,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        48,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        50,
        44,
        32,
        109,
        97,
    ]) by (compute_only);
}

pub proof fn css_text_block75(i: int)
    requires
        4800 <= i < 4864,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4800, 4864) == seq![
        114u8,
        107,
        46,
        116,
        51,
        52,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        54,
        32,
        123,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        52,
        102,
        52,
        54,
        101,
        53,
        59,
        32,
        125,
        10,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        49,
        44,
        32,
        109,
    ]) by (compute_only);
}

pub proof fn css_text_block76(i: int)
    requires
        4864 <= i < 4928,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4864, 4928) == seq![
        97u8,
        114,
        107,
        46,
        116,
        50,
        51,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        53,
        44,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        55,
        32,
        123,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        53,
        55,
        53,
        51,
        52,
        101,
        59,
        32,
        125,
        10,
    ]) by (compute_only);
}

pub proof fn css_text_block77(i: int)
    requires
        4928 <= i < 4992,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4928, 4992) == seq![
        109u8,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        48,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        48,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        98,
        102,
        100,
        98,
        102,
        101,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
    ]) by (compute_only);
}

pub proof fn css_text_block78(i: int)
    requires
        4992 <= i < 5056,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(4992, 5056) == seq![
        114u8,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
    ]) by (compute_only);
}

pub proof fn css_text_block79(i: int)
    requires
        5056 <= i < 5120,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(5056, 5120) == seq![
        117u8,
        110,
        100,
        58,
        32,
        35,
        102,
        101,
        102,
        48,
        56,
        97,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        58,
        104,
    ]) by (compute_only);
}

pub proof fn css_text_block80(i: int)
    requires
        5120 <= i < 5184,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(5120, 5184) == seq![
        111u8,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        101,
        57,
        100,
        53,
        102,
        102,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
    ]) by (compute_only);
}

pub proof fn css_text_block81(i: int)
    requires
        5184 <= i < 5248,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(5184, 5248) == seq![
        100u8,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        101,
        100,
        55,
        97,
        97,
        59,
        32,
        116,
        101,
        120,
        116,
    ]) by (compute_only);
}

pub proof fn css_text_block82(i: int)
    requires
        5248 <= i < 5312,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(5248, 5312) == seq![
        45u8,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        32,
        123,
        32,
        98,
        97,
    ]) by (compute_only);
}

pub proof fn css_text_block83(i: int)
    requires
        5312 <= i < 5376,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(5312, 5376) == seq![
        99u8,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        57,
        57,
        102,
        54,
        101,
        52,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
    ]) by (compute_only);
}

pub proof fn css_text_block84(i: int)
    requires
        5376 <= i < 5440,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(5376, 5440) == seq![
        46u8,
        116,
        53,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        53,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        101,
        99,
        100,
        100,
        51,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
    ]) by (compute_only);
}

pub proof fn css_text_block85(i: int)
    requires
        5440 <= i < 5504,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(5440, 5504) == seq![
        32u8,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        54,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        54,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        98,
        98,
        102,
        55,
        100,
        48,
        59,
    ]) by (compute_only);
}

pub proof fn css_text_block86(i: int)
    requires
        5504 <= i < 5568,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(5504, 5568) == seq![
        32u8,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        55,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        55,
    ]) by (compute_only);
}

pub proof fn css_text_block87(i: int)
    requires
        5568 <= i < 5632,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(5568, 5632) == seq![
        32u8,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        53,
        100,
        48,
        102,
        101,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
    ]) by (compute_only);
}

pub proof fn css_text_block88(i: int)
    requires
        5632 <= i < 5696,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(5632, 5696) == seq![
        40u8,
        109,
        97,
        114,
        107,
        46,
        116,
        56,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        56,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        97,
        53,
        102,
        51,
        102,
        99,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
    ]) by (compute_only);
}

pub proof fn css_text_block89(i: int)
    requires
        5696 <= i < 5760,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(5696, 5760) == seq![
        116u8,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        57,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        57,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        100,
        57,
    ]) by (compute_only);
}

pub proof fn css_text_block90(i: int)
    requires
        5760 <= i < 5824,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(5760, 5824) == seq![
        102u8,
        57,
        57,
        100,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        48,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
    ]) by (compute_only);
}

pub proof fn css_text_block91(i: int)
    requires
        5824 <= i < 5888,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(5824, 5888) == seq![
        97u8,
        114,
        107,
        46,
        116,
        49,
        48,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        99,
        55,
        100,
        50,
        102,
        101,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
    ]) by (compute_only);
}

pub proof fn css_text_block92(i: int)
    requires
        5888 <= i < 5952,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(5888, 5952) == seq![
        97u8,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        49,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        49,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        100,
        54,
        100,
        51,
        100,
        49,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
    ]) by (compute_only);
}

pub proof fn css_text_block93(i: int)
    requires
        5952 <= i < 6016,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(5952, 6016) == seq![
        111u8,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        50,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        50,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
    ]) by (compute_only);
}

pub proof fn css_text_block94(i: int)
    requires
        6016 <= i < 6080,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6016, 6080) == seq![
        103u8,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        98,
        102,
        100,
        98,
        102,
        101,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
    ]) by (compute_only);
}

pub proof fn css_text_block95(i: int)
    requires
        6080 <= i < 6144,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6080, 6144) == seq![
        49u8,
        51,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        51,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        101,
        102,
        48,
        56,
        97,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
    ]) by (compute_only);
}

pub proof fn css_text_block96(i: int)
    requires
        6144 <= i < 6208,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6144, 6208) == seq![
        32u8,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        52,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        52,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        101,
        57,
        100,
        53,
        102,
    ]) by (compute_only);
}

pub proof fn css_text_block97(i: int)
    requires
        6208 <= i < 6272,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6208, 6272) == seq![
        102u8,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        53,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
    ]) by (compute_only);
}

pub proof fn css_text_block98(i: int)
    requires
        6272 <= i < 6336,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6272, 6336) == seq![
        46u8,
        116,
        49,
        53,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        101,
        100,
        55,
        97,
        97,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
    ]) by (compute_only);
}

pub proof fn css_text_block99(i: int)
    requires
        6336 <= i < 6400,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6336, 6400) == seq![
        58u8,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        54,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        54,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        57,
        57,
        102,
        54,
        101,
        52,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
    ]) by (compute_only);
}

pub proof fn css_text_block100(i: int)
    requires
        6400 <= i < 6464,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6400, 6464) == seq![
        116u8,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        55,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        55,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
    ]) by (compute_only);
}

pub proof fn css_text_block101(i: int)
    requires
        6464 <= i < 6528,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6464, 6528) == seq![
        117u8,
        110,
        100,
        58,
        32,
        35,
        102,
        101,
        99,
        100,
        100,
        51,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        56,
        58,
    ]) by (compute_only);
}

pub proof fn css_text_block102(i: int)
    requires
        6528 <= i < 6592,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6528, 6592) == seq![
        104u8,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        56,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        98,
        98,
        102,
        55,
        100,
        48,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
    ]) by (compute_only);
}

pub proof fn css_text_block103(i: int)
    requires
        6592 <= i < 6656,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6592, 6656) == seq![
        108u8,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        57,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        49,
        57,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        53,
        100,
        48,
        102,
        101,
        59,
        32,
    ]) by (compute_only);
}

pub proof fn css_text_block104(i: int)
    requires
        6656 <= i < 6720,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6656, 6720) == seq![
        116u8,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        48,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
    ]) by (compute_only);
}

pub proof fn css_text_block105(i: int)
    requires
        6720 <= i < 6784,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6720, 6784) == seq![
        48u8,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        97,
        53,
        102,
        51,
        102,
        99,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
    ]) by (compute_only);
}

pub proof fn css_text_block106(i: int)
    requires
        6784 <= i < 6848,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6784, 6848) == seq![
        115u8,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        49,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        49,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        100,
        57,
        102,
        57,
        57,
        100,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
    ]) by (compute_only);
}

pub proof fn css_text_block107(i: int)
    requires
        6848 <= i < 6912,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6848, 6912) == seq![
        110u8,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        50,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        50,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
    ]) by (compute_only);
}

pub proof fn css_text_block108(i: int)
    requires
        6912 <= i < 6976,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6912, 6976) == seq![
        58u8,
        32,
        35,
        99,
        55,
        100,
        50,
        102,
        101,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        51,
        58,
        104,
        111,
        118,
    ]) by (compute_only);
}

pub proof fn css_text_block109(i: int)
    requires
        6976 <= i < 7040,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(6976, 7040) == seq![
        101u8,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        51,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        100,
        54,
        100,
        51,
        100,
        49,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
    ]) by (compute_only);
}

pub proof fn css_text_block110(i: int)
    requires
        7040 <= i < 7104,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(7040, 7104) == seq![
        59u8,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        52,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        52,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        98,
        102,
        100,
        98,
        102,
        101,
        59,
        32,
        116,
        101,
        120,
    ]) by (compute_only);
}

pub proof fn css_text_block111(i: int)
    requires
        7104 <= i < 7168,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(7104, 7168) == seq![
        116u8,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        53,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        53,
        32,
        123,
    ]) by (compute_only);
}

pub proof fn css_text_block112(i: int)
    requires
        7168 <= i < 7232,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(7168, 7232) == seq![
        32u8,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        101,
        102,
        48,
        56,
        97,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
    ]) by (compute_only);
}

pub proof fn css_text_block113(i: int)
    requires
        7232 <= i < 7296,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(7232, 7296) == seq![
        97u8,
        114,
        107,
        46,
        116,
        50,
        54,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        54,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        101,
        57,
        100,
        53,
        102,
        102,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
    ]) by (compute_only);
}

pub proof fn css_text_block114(i: int)
    requires
        7296 <= i < 7360,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(7296, 7360) == seq![
        116u8,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        55,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        55,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
    ]) by (compute_only);
}

pub proof fn css_text_block115(i: int)
    requires
        7360 <= i < 7424,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(7360, 7424) == seq![
        102u8,
        101,
        100,
        55,
        97,
        97,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        56,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
    ]) by (compute_only);
}

pub proof fn css_text_block116(i: int)
    requires
        7424 <= i < 7488,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(7424, 7488) == seq![
        32u8,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        56,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        57,
        57,
        102,
        54,
        101,
        52,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
    ]) by (compute_only);
}

pub proof fn css_text_block117(i: int)
    requires
        7488 <= i < 7552,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(7488, 7552) == seq![
        10u8,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        57,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        50,
        57,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        101,
        99,
        100,
        100,
        51,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
    ]) by (compute_only);
}

pub proof fn css_text_block118(i: int)
    requires
        7552 <= i < 7616,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(7552, 7616) == seq![
        101u8,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        48,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        48,
        32,
        123,
        32,
        98,
        97,
    ]) by (compute_only);
}

pub proof fn css_text_block119(i: int)
    requires
        7616 <= i < 7680,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(7616, 7680) == seq![
        99u8,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        98,
        98,
        102,
        55,
        100,
        48,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
    ]) by (compute_only);
}

pub proof fn css_text_block120(i: int)
    requires
        7680 <= i < 7744,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(7680, 7744) == seq![
        46u8,
        116,
        51,
        49,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        49,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        53,
        100,
        48,
        102,
        101,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
    ]) by (compute_only);
}

pub proof fn css_text_block121(i: int)
    requires
        7744 <= i < 7808,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(7744, 7808) == seq![
        101u8,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        50,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        50,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        97,
        53,
        102,
    ]) by (compute_only);
}

pub proof fn css_text_block122(i: int)
    requires
        7808 <= i < 7872,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(7808, 7872) == seq![
        51u8,
        102,
        99,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        51,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
    ]) by (compute_only);
}

pub proof fn css_text_block123(i: int)
    requires
        7872 <= i < 7936,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(7872, 7936) == seq![
        114u8,
        107,
        46,
        116,
        51,
        51,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        100,
        57,
        102,
        57,
        57,
        100,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
    ]) by (compute_only);
}

pub proof fn css_text_block124(i: int)
    requires
        7936 <= i < 8000,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(7936, 8000) == seq![
        105u8,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        52,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        52,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        99,
        55,
        100,
        50,
        102,
        101,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
    ]) by (compute_only);
}

pub proof fn css_text_block125(i: int)
    requires
        8000 <= i < 8064,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8000, 8064) == seq![
        114u8,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        53,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        53,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
    ]) by (compute_only);
}

pub proof fn css_text_block126(i: int)
    requires
        8064 <= i < 8128,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8064, 8128) == seq![
        114u8,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        100,
        54,
        100,
        51,
        100,
        49,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
    ]) by (compute_only);
}

pub proof fn css_text_block127(i: int)
    requires
        8128 <= i < 8192,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8128, 8192) == seq![
        54u8,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        54,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        98,
        102,
        100,
        98,
        102,
        101,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
    ]) by (compute_only);
}

pub proof fn css_text_block128(i: int)
    requires
        8192 <= i < 8256,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8192, 8256) == seq![
        115u8,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        55,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        55,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        101,
        102,
        48,
        56,
        97,
    ]) by (compute_only);
}

pub proof fn css_text_block129(i: int)
    requires
        8256 <= i < 8320,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8256, 8320) == seq![
        59u8,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        56,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
    ]) by (compute_only);
}

pub proof fn css_text_block130(i: int)
    requires
        8320 <= i < 8384,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8320, 8384) == seq![
        116u8,
        51,
        56,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        101,
        57,
        100,
        53,
        102,
        102,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
    ]) by (compute_only);
}

pub proof fn css_text_block131(i: int)
    requires
        8384 <= i < 8448,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8384, 8448) == seq![
        104u8,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        57,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        51,
        57,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        101,
        100,
        55,
        97,
        97,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
    ]) by (compute_only);
}

pub proof fn css_text_block132(i: int)
    requires
        8448 <= i < 8512,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8448, 8512) == seq![
        105u8,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        48,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        48,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
    ]) by (compute_only);
}

pub proof fn css_text_block133(i: int)
    requires
        8512 <= i < 8576,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8512, 8576) == seq![
        110u8,
        100,
        58,
        32,
        35,
        57,
        57,
        102,
        54,
        101,
        52,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        49,
        58,
        104,
    ]) by (compute_only);
}

pub proof fn css_text_block134(i: int)
    requires
        8576 <= i < 8640,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8576, 8640) == seq![
        111u8,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        49,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        101,
        99,
        100,
        100,
        51,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
    ]) by (compute_only);
}

pub proof fn css_text_block135(i: int)
    requires
        8640 <= i < 8704,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8640, 8704) == seq![
        105u8,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        50,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        50,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        98,
        98,
        102,
        55,
        100,
        48,
        59,
        32,
        116,
    ]) by (compute_only);
}

pub proof fn css_text_block136(i: int)
    requires
        8704 <= i < 8768,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8704, 8768) == seq![
        101u8,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        51,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        51,
    ]) by (compute_only);
}

pub proof fn css_text_block137(i: int)
    requires
        8768 <= i < 8832,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8768, 8832) == seq![
        32u8,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        102,
        53,
        100,
        48,
        102,
        101,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
    ]) by (compute_only);
}

pub proof fn css_text_block138(i: int)
    requires
        8832 <= i < 8896,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8832, 8896) == seq![
        40u8,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        52,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        52,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        97,
        53,
        102,
        51,
        102,
        99,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
    ]) by (compute_only);
}

pub proof fn css_text_block139(i: int)
    requires
        8896 <= i < 8960,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8896, 8960) == seq![
        45u8,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        53,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        53,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
    ]) by (compute_only);
}

pub proof fn css_text_block140(i: int)
    requires
        8960 <= i < 9024,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(8960, 9024) == seq![
        32u8,
        35,
        100,
        57,
        102,
        57,
        57,
        100,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        54,
        58,
        104,
        111,
        118,
        101,
    ]) by (compute_only);
}

pub proof fn css_text_block141(i: int)
    requires
        9024 <= i < 9088,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9024, 9088) == seq![
        114u8,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        54,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        99,
        55,
        100,
        50,
        102,
        101,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
    ]) by (compute_only);
}

pub proof fn css_text_block142(i: int)
    requires
        9088 <= i < 9152,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9088, 9152) == seq![
        32u8,
        125,
        10,
        109,
        97,
        105,
        110,
        58,
        104,
        97,
        115,
        40,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        55,
        58,
        104,
        111,
        118,
        101,
        114,
        41,
        32,
        109,
        97,
        114,
        107,
        46,
        116,
        52,
        55,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        35,
        100,
        54,
        100,
        51,
        100,
        49,
        59,
        32,
        116,
        101,
        120,
        116,
    ]) by (compute_only);
}

pub proof fn css_text_block143(i: int)
    requires
        9152 <= i < 9216,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9152, 9216) == seq![
        45u8,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        115,
        116,
        121,
        108,
        101,
        58,
        32,
        115,
        111,
        108,
        105,
        100,
        59,
        32,
        125,
        10,
        46,
        104,
        108,
        45,
        110,
        111,
        116,
        101,
        32,
        108,
        97,
        98,
        101,
        108,
        32,
        123,
        32,
        109,
        97,
        114,
        103,
        105,
        110,
        45,
        114,
        105,
        103,
        104,
        116,
        58,
        32,
        48,
        46,
        55,
        53,
        114,
    ]) by (compute_only);
}

pub proof fn css_text_block144(i: int)
    requires
        9216 <= i < 9280,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9216, 9280) == seq![
        101u8,
        109,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        46,
        104,
        108,
        45,
        99,
        108,
        105,
        99,
        107,
        32,
        112,
        114,
        101,
        46,
        112,
        114,
        111,
        115,
        101,
        32,
        123,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        57,
        99,
        97,
        51,
        97,
        102,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        46,
        104,
        108,
        45,
        99,
        108,
        105,
        99,
        107,
        32,
    ]) by (compute_only);
}

pub proof fn css_text_block145(i: int)
    requires
        9280 <= i < 9344,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9280, 9344) == seq![
        112u8,
        114,
        101,
        46,
        112,
        114,
        111,
        115,
        101,
        32,
        46,
        107,
        119,
        32,
        123,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        57,
        99,
        97,
        51,
        97,
        102,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        46,
        104,
        108,
        45,
        99,
        108,
        105,
        99,
        107,
        32,
        112,
        114,
        101,
        46,
        112,
        114,
        111,
        115,
        101,
        32,
        109,
        97,
        114,
        107,
        58,
        110,
    ]) by (compute_only);
}

pub proof fn css_text_block146(i: int)
    requires
        9344 <= i < 9408,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9344, 9408) == seq![
        111u8,
        116,
        40,
        46,
        104,
        108,
        45,
        112,
        105,
        99,
        107,
        41,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        110,
        111,
        110,
        101,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        57,
        99,
        97,
        51,
        97,
        102,
        59,
    ]) by (compute_only);
}

pub proof fn css_text_block147(i: int)
    requires
        9408 <= i < 9472,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9408, 9472) == seq![
        32u8,
        125,
        10,
        109,
        97,
        105,
        110,
        46,
        104,
        108,
        45,
        99,
        108,
        105,
        99,
        107,
        32,
        112,
        114,
        101,
        46,
        112,
        114,
        111,
        115,
        101,
        32,
        109,
        97,
        114,
        107,
        46,
        104,
        108,
        45,
        112,
        105,
        99,
        107,
        32,
        123,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        49,
        49,
        49,
        56,
        50,
        55,
        59,
        32,
        125,
        10,
        98,
        111,
        100,
        121,
    ]) by (compute_only);
}

pub proof fn css_text_block148(i: int)
    requires
        9472 <= i < 9536,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9472, 9536) == seq![
        58u8,
        104,
        97,
        115,
        40,
        105,
        110,
        112,
        117,
        116,
        46,
        104,
        108,
        45,
        116,
        111,
        103,
        103,
        108,
        101,
        58,
        110,
        111,
        116,
        40,
        58,
        99,
        104,
        101,
        99,
        107,
        101,
        100,
        41,
        41,
        32,
        112,
        114,
        101,
        46,
        112,
        114,
        111,
        115,
        101,
        32,
        109,
        97,
        114,
        107,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
    ]) by (compute_only);
}

pub proof fn css_text_block149(i: int)
    requires
        9536 <= i < 9600,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9536, 9600) == seq![
        32u8,
        110,
        111,
        110,
        101,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        58,
        32,
        110,
        111,
        110,
        101,
        59,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        105,
        110,
        104,
        101,
        114,
        105,
        116,
        59,
        32,
        125,
        10,
        98,
        111,
        100,
        121,
        58,
        104,
        97,
        115,
        40,
        105,
        110,
        112,
        117,
        116,
        46,
        104,
    ]) by (compute_only);
}

pub proof fn css_text_block150(i: int)
    requires
        9600 <= i < 9664,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9600, 9664) == seq![
        108u8,
        45,
        116,
        111,
        103,
        103,
        108,
        101,
        58,
        110,
        111,
        116,
        40,
        58,
        99,
        104,
        101,
        99,
        107,
        101,
        100,
        41,
        41,
        32,
        112,
        114,
        101,
        46,
        112,
        114,
        111,
        115,
        101,
        32,
        46,
        107,
        119,
        32,
        123,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        105,
        110,
        104,
        101,
        114,
        105,
        116,
        59,
        32,
        125,
        10,
        64,
        109,
        101,
        100,
        105,
        97,
    ]) by (compute_only);
}

pub proof fn css_text_block151(i: int)
    requires
        9664 <= i < 9728,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9664, 9728) == seq![
        32u8,
        112,
        114,
        105,
        110,
        116,
        32,
        123,
        10,
        98,
        111,
        100,
        121,
        32,
        123,
        32,
        109,
        97,
        120,
        45,
        119,
        105,
        100,
        116,
        104,
        58,
        32,
        110,
        111,
        110,
        101,
        59,
        32,
        112,
        97,
        100,
        100,
        105,
        110,
        103,
        58,
        32,
        48,
        59,
        32,
        125,
        10,
        110,
        97,
        118,
        46,
        99,
        114,
        117,
        109,
        98,
        115,
        44,
        32,
        110,
        97,
        118,
        46,
        100,
    ]) by (compute_only);
}

pub proof fn css_text_block152(i: int)
    requires
        9728 <= i < 9792,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9728, 9792) == seq![
        111u8,
        99,
        110,
        97,
        118,
        44,
        32,
        46,
        115,
        107,
        105,
        112,
        44,
        32,
        102,
        111,
        114,
        109,
        44,
        32,
        46,
        118,
        101,
        114,
        100,
        105,
        99,
        116,
        45,
        101,
        110,
        116,
        114,
        121,
        44,
        32,
        46,
        104,
        108,
        45,
        110,
        111,
        116,
        101,
        32,
        123,
        32,
        100,
        105,
        115,
        112,
        108,
        97,
        121,
        58,
        32,
        110,
        111,
        110,
        101,
        59,
        32,
        125,
        10,
    ]) by (compute_only);
}

pub proof fn css_text_block153(i: int)
    requires
        9792 <= i < 9856,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9792, 9856) == seq![
        109u8,
        97,
        114,
        107,
        44,
        32,
        109,
        97,
        114,
        107,
        91,
        99,
        108,
        97,
        115,
        115,
        93,
        32,
        123,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        110,
        111,
        110,
        101,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
        116,
        105,
        111,
        110,
        45,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        52,
        98,
    ]) by (compute_only);
}

pub proof fn css_text_block154(i: int)
    requires
        9856 <= i < 9920,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9856, 9920) == seq![
        53u8,
        53,
        54,
        51,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        46,
        104,
        108,
        45,
        99,
        108,
        105,
        99,
        107,
        32,
        112,
        114,
        101,
        46,
        112,
        114,
        111,
        115,
        101,
        44,
        32,
        109,
        97,
        105,
        110,
        46,
        104,
        108,
        45,
        99,
        108,
        105,
        99,
        107,
        32,
        112,
        114,
        101,
        46,
        112,
        114,
        111,
        115,
        101,
        32,
        109,
        97,
        114,
        107,
        46,
        104,
        108,
    ]) by (compute_only);
}

pub proof fn css_text_block155(i: int)
    requires
        9920 <= i < 9984,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9920, 9984) == seq![
        45u8,
        112,
        105,
        99,
        107,
        32,
        123,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        105,
        110,
        104,
        101,
        114,
        105,
        116,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        46,
        104,
        108,
        45,
        99,
        108,
        105,
        99,
        107,
        32,
        112,
        114,
        101,
        46,
        112,
        114,
        111,
        115,
        101,
        32,
        46,
        107,
        119,
        32,
        123,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
    ]) by (compute_only);
}

pub proof fn css_text_block156(i: int)
    requires
        9984 <= i < 10048,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(9984, 10048) == seq![
        52u8,
        98,
        53,
        53,
        54,
        51,
        59,
        32,
        125,
        10,
        109,
        97,
        105,
        110,
        46,
        104,
        108,
        45,
        99,
        108,
        105,
        99,
        107,
        32,
        112,
        114,
        101,
        46,
        112,
        114,
        111,
        115,
        101,
        32,
        109,
        97,
        114,
        107,
        58,
        110,
        111,
        116,
        40,
        46,
        104,
        108,
        45,
        112,
        105,
        99,
        107,
        41,
        32,
        123,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
    ]) by (compute_only);
}

pub proof fn css_text_block157(i: int)
    requires
        10048 <= i < 10112,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(10048, 10112) == seq![
        114u8,
        97,
        116,
        105,
        111,
        110,
        45,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        35,
        52,
        98,
        53,
        53,
        54,
        51,
        59,
        32,
        125,
        10,
        100,
        101,
        116,
        97,
        105,
        108,
        115,
        58,
        58,
        100,
        101,
        116,
        97,
        105,
        108,
        115,
        45,
        99,
        111,
        110,
        116,
        101,
        110,
        116,
        32,
        123,
        32,
        99,
        111,
        110,
        116,
        101,
        110,
        116,
        45,
        118,
        105,
        115,
        105,
    ]) by (compute_only);
}

pub proof fn css_text_block158(i: int)
    requires
        10112 <= i < 10176,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(10112, 10176) == seq![
        98u8,
        105,
        108,
        105,
        116,
        121,
        58,
        32,
        118,
        105,
        115,
        105,
        98,
        108,
        101,
        59,
        32,
        125,
        10,
        112,
        114,
        101,
        32,
        123,
        32,
        98,
        111,
        114,
        100,
        101,
        114,
        58,
        32,
        110,
        111,
        110,
        101,
        59,
        32,
        112,
        97,
        100,
        100,
        105,
        110,
        103,
        58,
        32,
        48,
        59,
        32,
        119,
        104,
        105,
        116,
        101,
        45,
        115,
        112,
        97,
        99,
        101,
        58,
        32,
    ]) by (compute_only);
}

pub proof fn css_text_block159(i: int)
    requires
        10176 <= i < 10240,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(10176, 10240) == seq![
        112u8,
        114,
        101,
        45,
        119,
        114,
        97,
        112,
        59,
        32,
        111,
        118,
        101,
        114,
        102,
        108,
        111,
        119,
        45,
        120,
        58,
        32,
        118,
        105,
        115,
        105,
        98,
        108,
        101,
        59,
        32,
        125,
        10,
        97,
        32,
        123,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        105,
        110,
        104,
        101,
        114,
        105,
        116,
        59,
        32,
        116,
        101,
        120,
        116,
        45,
        100,
        101,
        99,
        111,
        114,
        97,
    ]) by (compute_only);
}

pub proof fn css_text_block160(i: int)
    requires
        10240 <= i < 10304,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(10240, 10304) == seq![
        116u8,
        105,
        111,
        110,
        58,
        32,
        110,
        111,
        110,
        101,
        59,
        32,
        125,
        10,
        46,
        99,
        104,
        105,
        112,
        32,
        123,
        32,
        98,
        111,
        114,
        100,
        101,
        114,
        58,
        32,
        49,
        112,
        120,
        32,
        115,
        111,
        108,
        105,
        100,
        32,
        35,
        49,
        49,
        49,
        56,
        50,
        55,
        59,
        32,
        98,
        97,
        99,
        107,
        103,
        114,
        111,
        117,
        110,
        100,
        58,
        32,
        110,
        111,
        110,
    ]) by (compute_only);
}

pub proof fn css_text_block161(i: int)
    requires
        10304 <= i < 10326,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
    assert(ch::css_text_bytes().subrange(10304, 10326) == seq![
        101u8,
        59,
        32,
        99,
        111,
        108,
        111,
        114,
        58,
        32,
        105,
        110,
        104,
        101,
        114,
        105,
        116,
        59,
        32,
        125,
        10,
        125,
    ]) by (compute_only);
}

pub proof fn css_text_range_0_1(i: int)
    requires
        0 <= i < 64,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block0(i);
}

pub proof fn css_text_range_1_2(i: int)
    requires
        64 <= i < 128,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block1(i);
}

pub proof fn css_text_range_0_2(i: int)
    requires
        0 <= i < 128,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 64 {
        css_text_range_0_1(i);
    } else {
        css_text_range_1_2(i);
    }
}

pub proof fn css_text_range_2_3(i: int)
    requires
        128 <= i < 192,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block2(i);
}

pub proof fn css_text_range_3_4(i: int)
    requires
        192 <= i < 256,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block3(i);
}

pub proof fn css_text_range_4_5(i: int)
    requires
        256 <= i < 320,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block4(i);
}

pub proof fn css_text_range_3_5(i: int)
    requires
        192 <= i < 320,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 256 {
        css_text_range_3_4(i);
    } else {
        css_text_range_4_5(i);
    }
}

pub proof fn css_text_range_2_5(i: int)
    requires
        128 <= i < 320,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 192 {
        css_text_range_2_3(i);
    } else {
        css_text_range_3_5(i);
    }
}

pub proof fn css_text_range_0_5(i: int)
    requires
        0 <= i < 320,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 128 {
        css_text_range_0_2(i);
    } else {
        css_text_range_2_5(i);
    }
}

pub proof fn css_text_range_5_6(i: int)
    requires
        320 <= i < 384,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block5(i);
}

pub proof fn css_text_range_6_7(i: int)
    requires
        384 <= i < 448,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block6(i);
}

pub proof fn css_text_range_5_7(i: int)
    requires
        320 <= i < 448,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 384 {
        css_text_range_5_6(i);
    } else {
        css_text_range_6_7(i);
    }
}

pub proof fn css_text_range_7_8(i: int)
    requires
        448 <= i < 512,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block7(i);
}

pub proof fn css_text_range_8_9(i: int)
    requires
        512 <= i < 576,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block8(i);
}

pub proof fn css_text_range_9_10(i: int)
    requires
        576 <= i < 640,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block9(i);
}

pub proof fn css_text_range_8_10(i: int)
    requires
        512 <= i < 640,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 576 {
        css_text_range_8_9(i);
    } else {
        css_text_range_9_10(i);
    }
}

pub proof fn css_text_range_7_10(i: int)
    requires
        448 <= i < 640,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 512 {
        css_text_range_7_8(i);
    } else {
        css_text_range_8_10(i);
    }
}

pub proof fn css_text_range_5_10(i: int)
    requires
        320 <= i < 640,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 448 {
        css_text_range_5_7(i);
    } else {
        css_text_range_7_10(i);
    }
}

pub proof fn css_text_range_0_10(i: int)
    requires
        0 <= i < 640,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 320 {
        css_text_range_0_5(i);
    } else {
        css_text_range_5_10(i);
    }
}

pub proof fn css_text_range_10_11(i: int)
    requires
        640 <= i < 704,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block10(i);
}

pub proof fn css_text_range_11_12(i: int)
    requires
        704 <= i < 768,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block11(i);
}

pub proof fn css_text_range_10_12(i: int)
    requires
        640 <= i < 768,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 704 {
        css_text_range_10_11(i);
    } else {
        css_text_range_11_12(i);
    }
}

pub proof fn css_text_range_12_13(i: int)
    requires
        768 <= i < 832,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block12(i);
}

pub proof fn css_text_range_13_14(i: int)
    requires
        832 <= i < 896,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block13(i);
}

pub proof fn css_text_range_14_15(i: int)
    requires
        896 <= i < 960,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block14(i);
}

pub proof fn css_text_range_13_15(i: int)
    requires
        832 <= i < 960,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 896 {
        css_text_range_13_14(i);
    } else {
        css_text_range_14_15(i);
    }
}

pub proof fn css_text_range_12_15(i: int)
    requires
        768 <= i < 960,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 832 {
        css_text_range_12_13(i);
    } else {
        css_text_range_13_15(i);
    }
}

pub proof fn css_text_range_10_15(i: int)
    requires
        640 <= i < 960,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 768 {
        css_text_range_10_12(i);
    } else {
        css_text_range_12_15(i);
    }
}

pub proof fn css_text_range_15_16(i: int)
    requires
        960 <= i < 1024,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block15(i);
}

pub proof fn css_text_range_16_17(i: int)
    requires
        1024 <= i < 1088,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block16(i);
}

pub proof fn css_text_range_15_17(i: int)
    requires
        960 <= i < 1088,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1024 {
        css_text_range_15_16(i);
    } else {
        css_text_range_16_17(i);
    }
}

pub proof fn css_text_range_17_18(i: int)
    requires
        1088 <= i < 1152,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block17(i);
}

pub proof fn css_text_range_18_19(i: int)
    requires
        1152 <= i < 1216,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block18(i);
}

pub proof fn css_text_range_19_20(i: int)
    requires
        1216 <= i < 1280,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block19(i);
}

pub proof fn css_text_range_18_20(i: int)
    requires
        1152 <= i < 1280,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1216 {
        css_text_range_18_19(i);
    } else {
        css_text_range_19_20(i);
    }
}

pub proof fn css_text_range_17_20(i: int)
    requires
        1088 <= i < 1280,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1152 {
        css_text_range_17_18(i);
    } else {
        css_text_range_18_20(i);
    }
}

pub proof fn css_text_range_15_20(i: int)
    requires
        960 <= i < 1280,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1088 {
        css_text_range_15_17(i);
    } else {
        css_text_range_17_20(i);
    }
}

pub proof fn css_text_range_10_20(i: int)
    requires
        640 <= i < 1280,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 960 {
        css_text_range_10_15(i);
    } else {
        css_text_range_15_20(i);
    }
}

pub proof fn css_text_range_0_20(i: int)
    requires
        0 <= i < 1280,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 640 {
        css_text_range_0_10(i);
    } else {
        css_text_range_10_20(i);
    }
}

pub proof fn css_text_range_20_21(i: int)
    requires
        1280 <= i < 1344,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block20(i);
}

pub proof fn css_text_range_21_22(i: int)
    requires
        1344 <= i < 1408,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block21(i);
}

pub proof fn css_text_range_20_22(i: int)
    requires
        1280 <= i < 1408,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1344 {
        css_text_range_20_21(i);
    } else {
        css_text_range_21_22(i);
    }
}

pub proof fn css_text_range_22_23(i: int)
    requires
        1408 <= i < 1472,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block22(i);
}

pub proof fn css_text_range_23_24(i: int)
    requires
        1472 <= i < 1536,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block23(i);
}

pub proof fn css_text_range_24_25(i: int)
    requires
        1536 <= i < 1600,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block24(i);
}

pub proof fn css_text_range_23_25(i: int)
    requires
        1472 <= i < 1600,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1536 {
        css_text_range_23_24(i);
    } else {
        css_text_range_24_25(i);
    }
}

pub proof fn css_text_range_22_25(i: int)
    requires
        1408 <= i < 1600,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1472 {
        css_text_range_22_23(i);
    } else {
        css_text_range_23_25(i);
    }
}

pub proof fn css_text_range_20_25(i: int)
    requires
        1280 <= i < 1600,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1408 {
        css_text_range_20_22(i);
    } else {
        css_text_range_22_25(i);
    }
}

pub proof fn css_text_range_25_26(i: int)
    requires
        1600 <= i < 1664,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block25(i);
}

pub proof fn css_text_range_26_27(i: int)
    requires
        1664 <= i < 1728,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block26(i);
}

pub proof fn css_text_range_25_27(i: int)
    requires
        1600 <= i < 1728,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1664 {
        css_text_range_25_26(i);
    } else {
        css_text_range_26_27(i);
    }
}

pub proof fn css_text_range_27_28(i: int)
    requires
        1728 <= i < 1792,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block27(i);
}

pub proof fn css_text_range_28_29(i: int)
    requires
        1792 <= i < 1856,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block28(i);
}

pub proof fn css_text_range_29_30(i: int)
    requires
        1856 <= i < 1920,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block29(i);
}

pub proof fn css_text_range_28_30(i: int)
    requires
        1792 <= i < 1920,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1856 {
        css_text_range_28_29(i);
    } else {
        css_text_range_29_30(i);
    }
}

pub proof fn css_text_range_27_30(i: int)
    requires
        1728 <= i < 1920,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1792 {
        css_text_range_27_28(i);
    } else {
        css_text_range_28_30(i);
    }
}

pub proof fn css_text_range_25_30(i: int)
    requires
        1600 <= i < 1920,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1728 {
        css_text_range_25_27(i);
    } else {
        css_text_range_27_30(i);
    }
}

pub proof fn css_text_range_20_30(i: int)
    requires
        1280 <= i < 1920,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1600 {
        css_text_range_20_25(i);
    } else {
        css_text_range_25_30(i);
    }
}

pub proof fn css_text_range_30_31(i: int)
    requires
        1920 <= i < 1984,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block30(i);
}

pub proof fn css_text_range_31_32(i: int)
    requires
        1984 <= i < 2048,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block31(i);
}

pub proof fn css_text_range_30_32(i: int)
    requires
        1920 <= i < 2048,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1984 {
        css_text_range_30_31(i);
    } else {
        css_text_range_31_32(i);
    }
}

pub proof fn css_text_range_32_33(i: int)
    requires
        2048 <= i < 2112,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block32(i);
}

pub proof fn css_text_range_33_34(i: int)
    requires
        2112 <= i < 2176,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block33(i);
}

pub proof fn css_text_range_34_35(i: int)
    requires
        2176 <= i < 2240,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block34(i);
}

pub proof fn css_text_range_33_35(i: int)
    requires
        2112 <= i < 2240,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 2176 {
        css_text_range_33_34(i);
    } else {
        css_text_range_34_35(i);
    }
}

pub proof fn css_text_range_32_35(i: int)
    requires
        2048 <= i < 2240,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 2112 {
        css_text_range_32_33(i);
    } else {
        css_text_range_33_35(i);
    }
}

pub proof fn css_text_range_30_35(i: int)
    requires
        1920 <= i < 2240,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 2048 {
        css_text_range_30_32(i);
    } else {
        css_text_range_32_35(i);
    }
}

pub proof fn css_text_range_35_36(i: int)
    requires
        2240 <= i < 2304,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block35(i);
}

pub proof fn css_text_range_36_37(i: int)
    requires
        2304 <= i < 2368,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block36(i);
}

pub proof fn css_text_range_35_37(i: int)
    requires
        2240 <= i < 2368,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 2304 {
        css_text_range_35_36(i);
    } else {
        css_text_range_36_37(i);
    }
}

pub proof fn css_text_range_37_38(i: int)
    requires
        2368 <= i < 2432,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block37(i);
}

pub proof fn css_text_range_38_39(i: int)
    requires
        2432 <= i < 2496,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block38(i);
}

pub proof fn css_text_range_39_40(i: int)
    requires
        2496 <= i < 2560,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block39(i);
}

pub proof fn css_text_range_38_40(i: int)
    requires
        2432 <= i < 2560,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 2496 {
        css_text_range_38_39(i);
    } else {
        css_text_range_39_40(i);
    }
}

pub proof fn css_text_range_37_40(i: int)
    requires
        2368 <= i < 2560,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 2432 {
        css_text_range_37_38(i);
    } else {
        css_text_range_38_40(i);
    }
}

pub proof fn css_text_range_35_40(i: int)
    requires
        2240 <= i < 2560,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 2368 {
        css_text_range_35_37(i);
    } else {
        css_text_range_37_40(i);
    }
}

pub proof fn css_text_range_30_40(i: int)
    requires
        1920 <= i < 2560,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 2240 {
        css_text_range_30_35(i);
    } else {
        css_text_range_35_40(i);
    }
}

pub proof fn css_text_range_20_40(i: int)
    requires
        1280 <= i < 2560,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1920 {
        css_text_range_20_30(i);
    } else {
        css_text_range_30_40(i);
    }
}

pub proof fn css_text_range_0_40(i: int)
    requires
        0 <= i < 2560,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 1280 {
        css_text_range_0_20(i);
    } else {
        css_text_range_20_40(i);
    }
}

pub proof fn css_text_range_40_41(i: int)
    requires
        2560 <= i < 2624,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block40(i);
}

pub proof fn css_text_range_41_42(i: int)
    requires
        2624 <= i < 2688,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block41(i);
}

pub proof fn css_text_range_40_42(i: int)
    requires
        2560 <= i < 2688,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 2624 {
        css_text_range_40_41(i);
    } else {
        css_text_range_41_42(i);
    }
}

pub proof fn css_text_range_42_43(i: int)
    requires
        2688 <= i < 2752,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block42(i);
}

pub proof fn css_text_range_43_44(i: int)
    requires
        2752 <= i < 2816,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block43(i);
}

pub proof fn css_text_range_44_45(i: int)
    requires
        2816 <= i < 2880,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block44(i);
}

pub proof fn css_text_range_43_45(i: int)
    requires
        2752 <= i < 2880,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 2816 {
        css_text_range_43_44(i);
    } else {
        css_text_range_44_45(i);
    }
}

pub proof fn css_text_range_42_45(i: int)
    requires
        2688 <= i < 2880,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 2752 {
        css_text_range_42_43(i);
    } else {
        css_text_range_43_45(i);
    }
}

pub proof fn css_text_range_40_45(i: int)
    requires
        2560 <= i < 2880,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 2688 {
        css_text_range_40_42(i);
    } else {
        css_text_range_42_45(i);
    }
}

pub proof fn css_text_range_45_46(i: int)
    requires
        2880 <= i < 2944,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block45(i);
}

pub proof fn css_text_range_46_47(i: int)
    requires
        2944 <= i < 3008,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block46(i);
}

pub proof fn css_text_range_45_47(i: int)
    requires
        2880 <= i < 3008,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 2944 {
        css_text_range_45_46(i);
    } else {
        css_text_range_46_47(i);
    }
}

pub proof fn css_text_range_47_48(i: int)
    requires
        3008 <= i < 3072,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block47(i);
}

pub proof fn css_text_range_48_49(i: int)
    requires
        3072 <= i < 3136,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block48(i);
}

pub proof fn css_text_range_49_50(i: int)
    requires
        3136 <= i < 3200,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block49(i);
}

pub proof fn css_text_range_48_50(i: int)
    requires
        3072 <= i < 3200,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3136 {
        css_text_range_48_49(i);
    } else {
        css_text_range_49_50(i);
    }
}

pub proof fn css_text_range_47_50(i: int)
    requires
        3008 <= i < 3200,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3072 {
        css_text_range_47_48(i);
    } else {
        css_text_range_48_50(i);
    }
}

pub proof fn css_text_range_45_50(i: int)
    requires
        2880 <= i < 3200,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3008 {
        css_text_range_45_47(i);
    } else {
        css_text_range_47_50(i);
    }
}

pub proof fn css_text_range_40_50(i: int)
    requires
        2560 <= i < 3200,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 2880 {
        css_text_range_40_45(i);
    } else {
        css_text_range_45_50(i);
    }
}

pub proof fn css_text_range_50_51(i: int)
    requires
        3200 <= i < 3264,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block50(i);
}

pub proof fn css_text_range_51_52(i: int)
    requires
        3264 <= i < 3328,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block51(i);
}

pub proof fn css_text_range_50_52(i: int)
    requires
        3200 <= i < 3328,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3264 {
        css_text_range_50_51(i);
    } else {
        css_text_range_51_52(i);
    }
}

pub proof fn css_text_range_52_53(i: int)
    requires
        3328 <= i < 3392,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block52(i);
}

pub proof fn css_text_range_53_54(i: int)
    requires
        3392 <= i < 3456,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block53(i);
}

pub proof fn css_text_range_54_55(i: int)
    requires
        3456 <= i < 3520,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block54(i);
}

pub proof fn css_text_range_53_55(i: int)
    requires
        3392 <= i < 3520,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3456 {
        css_text_range_53_54(i);
    } else {
        css_text_range_54_55(i);
    }
}

pub proof fn css_text_range_52_55(i: int)
    requires
        3328 <= i < 3520,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3392 {
        css_text_range_52_53(i);
    } else {
        css_text_range_53_55(i);
    }
}

pub proof fn css_text_range_50_55(i: int)
    requires
        3200 <= i < 3520,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3328 {
        css_text_range_50_52(i);
    } else {
        css_text_range_52_55(i);
    }
}

pub proof fn css_text_range_55_56(i: int)
    requires
        3520 <= i < 3584,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block55(i);
}

pub proof fn css_text_range_56_57(i: int)
    requires
        3584 <= i < 3648,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block56(i);
}

pub proof fn css_text_range_55_57(i: int)
    requires
        3520 <= i < 3648,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3584 {
        css_text_range_55_56(i);
    } else {
        css_text_range_56_57(i);
    }
}

pub proof fn css_text_range_57_58(i: int)
    requires
        3648 <= i < 3712,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block57(i);
}

pub proof fn css_text_range_58_59(i: int)
    requires
        3712 <= i < 3776,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block58(i);
}

pub proof fn css_text_range_59_60(i: int)
    requires
        3776 <= i < 3840,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block59(i);
}

pub proof fn css_text_range_58_60(i: int)
    requires
        3712 <= i < 3840,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3776 {
        css_text_range_58_59(i);
    } else {
        css_text_range_59_60(i);
    }
}

pub proof fn css_text_range_57_60(i: int)
    requires
        3648 <= i < 3840,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3712 {
        css_text_range_57_58(i);
    } else {
        css_text_range_58_60(i);
    }
}

pub proof fn css_text_range_55_60(i: int)
    requires
        3520 <= i < 3840,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3648 {
        css_text_range_55_57(i);
    } else {
        css_text_range_57_60(i);
    }
}

pub proof fn css_text_range_50_60(i: int)
    requires
        3200 <= i < 3840,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3520 {
        css_text_range_50_55(i);
    } else {
        css_text_range_55_60(i);
    }
}

pub proof fn css_text_range_40_60(i: int)
    requires
        2560 <= i < 3840,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3200 {
        css_text_range_40_50(i);
    } else {
        css_text_range_50_60(i);
    }
}

pub proof fn css_text_range_60_61(i: int)
    requires
        3840 <= i < 3904,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block60(i);
}

pub proof fn css_text_range_61_62(i: int)
    requires
        3904 <= i < 3968,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block61(i);
}

pub proof fn css_text_range_60_62(i: int)
    requires
        3840 <= i < 3968,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3904 {
        css_text_range_60_61(i);
    } else {
        css_text_range_61_62(i);
    }
}

pub proof fn css_text_range_62_63(i: int)
    requires
        3968 <= i < 4032,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block62(i);
}

pub proof fn css_text_range_63_64(i: int)
    requires
        4032 <= i < 4096,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block63(i);
}

pub proof fn css_text_range_64_65(i: int)
    requires
        4096 <= i < 4160,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block64(i);
}

pub proof fn css_text_range_63_65(i: int)
    requires
        4032 <= i < 4160,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4096 {
        css_text_range_63_64(i);
    } else {
        css_text_range_64_65(i);
    }
}

pub proof fn css_text_range_62_65(i: int)
    requires
        3968 <= i < 4160,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4032 {
        css_text_range_62_63(i);
    } else {
        css_text_range_63_65(i);
    }
}

pub proof fn css_text_range_60_65(i: int)
    requires
        3840 <= i < 4160,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3968 {
        css_text_range_60_62(i);
    } else {
        css_text_range_62_65(i);
    }
}

pub proof fn css_text_range_65_66(i: int)
    requires
        4160 <= i < 4224,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block65(i);
}

pub proof fn css_text_range_66_67(i: int)
    requires
        4224 <= i < 4288,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block66(i);
}

pub proof fn css_text_range_65_67(i: int)
    requires
        4160 <= i < 4288,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4224 {
        css_text_range_65_66(i);
    } else {
        css_text_range_66_67(i);
    }
}

pub proof fn css_text_range_67_68(i: int)
    requires
        4288 <= i < 4352,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block67(i);
}

pub proof fn css_text_range_68_69(i: int)
    requires
        4352 <= i < 4416,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block68(i);
}

pub proof fn css_text_range_69_70(i: int)
    requires
        4416 <= i < 4480,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block69(i);
}

pub proof fn css_text_range_68_70(i: int)
    requires
        4352 <= i < 4480,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4416 {
        css_text_range_68_69(i);
    } else {
        css_text_range_69_70(i);
    }
}

pub proof fn css_text_range_67_70(i: int)
    requires
        4288 <= i < 4480,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4352 {
        css_text_range_67_68(i);
    } else {
        css_text_range_68_70(i);
    }
}

pub proof fn css_text_range_65_70(i: int)
    requires
        4160 <= i < 4480,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4288 {
        css_text_range_65_67(i);
    } else {
        css_text_range_67_70(i);
    }
}

pub proof fn css_text_range_60_70(i: int)
    requires
        3840 <= i < 4480,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4160 {
        css_text_range_60_65(i);
    } else {
        css_text_range_65_70(i);
    }
}

pub proof fn css_text_range_70_71(i: int)
    requires
        4480 <= i < 4544,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block70(i);
}

pub proof fn css_text_range_71_72(i: int)
    requires
        4544 <= i < 4608,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block71(i);
}

pub proof fn css_text_range_70_72(i: int)
    requires
        4480 <= i < 4608,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4544 {
        css_text_range_70_71(i);
    } else {
        css_text_range_71_72(i);
    }
}

pub proof fn css_text_range_72_73(i: int)
    requires
        4608 <= i < 4672,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block72(i);
}

pub proof fn css_text_range_73_74(i: int)
    requires
        4672 <= i < 4736,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block73(i);
}

pub proof fn css_text_range_74_75(i: int)
    requires
        4736 <= i < 4800,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block74(i);
}

pub proof fn css_text_range_73_75(i: int)
    requires
        4672 <= i < 4800,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4736 {
        css_text_range_73_74(i);
    } else {
        css_text_range_74_75(i);
    }
}

pub proof fn css_text_range_72_75(i: int)
    requires
        4608 <= i < 4800,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4672 {
        css_text_range_72_73(i);
    } else {
        css_text_range_73_75(i);
    }
}

pub proof fn css_text_range_70_75(i: int)
    requires
        4480 <= i < 4800,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4608 {
        css_text_range_70_72(i);
    } else {
        css_text_range_72_75(i);
    }
}

pub proof fn css_text_range_75_76(i: int)
    requires
        4800 <= i < 4864,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block75(i);
}

pub proof fn css_text_range_76_77(i: int)
    requires
        4864 <= i < 4928,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block76(i);
}

pub proof fn css_text_range_77_78(i: int)
    requires
        4928 <= i < 4992,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block77(i);
}

pub proof fn css_text_range_76_78(i: int)
    requires
        4864 <= i < 4992,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4928 {
        css_text_range_76_77(i);
    } else {
        css_text_range_77_78(i);
    }
}

pub proof fn css_text_range_75_78(i: int)
    requires
        4800 <= i < 4992,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4864 {
        css_text_range_75_76(i);
    } else {
        css_text_range_76_78(i);
    }
}

pub proof fn css_text_range_78_79(i: int)
    requires
        4992 <= i < 5056,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block78(i);
}

pub proof fn css_text_range_79_80(i: int)
    requires
        5056 <= i < 5120,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block79(i);
}

pub proof fn css_text_range_80_81(i: int)
    requires
        5120 <= i < 5184,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block80(i);
}

pub proof fn css_text_range_79_81(i: int)
    requires
        5056 <= i < 5184,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 5120 {
        css_text_range_79_80(i);
    } else {
        css_text_range_80_81(i);
    }
}

pub proof fn css_text_range_78_81(i: int)
    requires
        4992 <= i < 5184,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 5056 {
        css_text_range_78_79(i);
    } else {
        css_text_range_79_81(i);
    }
}

pub proof fn css_text_range_75_81(i: int)
    requires
        4800 <= i < 5184,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4992 {
        css_text_range_75_78(i);
    } else {
        css_text_range_78_81(i);
    }
}

pub proof fn css_text_range_70_81(i: int)
    requires
        4480 <= i < 5184,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4800 {
        css_text_range_70_75(i);
    } else {
        css_text_range_75_81(i);
    }
}

pub proof fn css_text_range_60_81(i: int)
    requires
        3840 <= i < 5184,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 4480 {
        css_text_range_60_70(i);
    } else {
        css_text_range_70_81(i);
    }
}

pub proof fn css_text_range_40_81(i: int)
    requires
        2560 <= i < 5184,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 3840 {
        css_text_range_40_60(i);
    } else {
        css_text_range_60_81(i);
    }
}

pub proof fn css_text_range_0_81(i: int)
    requires
        0 <= i < 5184,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 2560 {
        css_text_range_0_40(i);
    } else {
        css_text_range_40_81(i);
    }
}

pub proof fn css_text_range_81_82(i: int)
    requires
        5184 <= i < 5248,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block81(i);
}

pub proof fn css_text_range_82_83(i: int)
    requires
        5248 <= i < 5312,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block82(i);
}

pub proof fn css_text_range_81_83(i: int)
    requires
        5184 <= i < 5312,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 5248 {
        css_text_range_81_82(i);
    } else {
        css_text_range_82_83(i);
    }
}

pub proof fn css_text_range_83_84(i: int)
    requires
        5312 <= i < 5376,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block83(i);
}

pub proof fn css_text_range_84_85(i: int)
    requires
        5376 <= i < 5440,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block84(i);
}

pub proof fn css_text_range_85_86(i: int)
    requires
        5440 <= i < 5504,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block85(i);
}

pub proof fn css_text_range_84_86(i: int)
    requires
        5376 <= i < 5504,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 5440 {
        css_text_range_84_85(i);
    } else {
        css_text_range_85_86(i);
    }
}

pub proof fn css_text_range_83_86(i: int)
    requires
        5312 <= i < 5504,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 5376 {
        css_text_range_83_84(i);
    } else {
        css_text_range_84_86(i);
    }
}

pub proof fn css_text_range_81_86(i: int)
    requires
        5184 <= i < 5504,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 5312 {
        css_text_range_81_83(i);
    } else {
        css_text_range_83_86(i);
    }
}

pub proof fn css_text_range_86_87(i: int)
    requires
        5504 <= i < 5568,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block86(i);
}

pub proof fn css_text_range_87_88(i: int)
    requires
        5568 <= i < 5632,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block87(i);
}

pub proof fn css_text_range_86_88(i: int)
    requires
        5504 <= i < 5632,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 5568 {
        css_text_range_86_87(i);
    } else {
        css_text_range_87_88(i);
    }
}

pub proof fn css_text_range_88_89(i: int)
    requires
        5632 <= i < 5696,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block88(i);
}

pub proof fn css_text_range_89_90(i: int)
    requires
        5696 <= i < 5760,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block89(i);
}

pub proof fn css_text_range_90_91(i: int)
    requires
        5760 <= i < 5824,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block90(i);
}

pub proof fn css_text_range_89_91(i: int)
    requires
        5696 <= i < 5824,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 5760 {
        css_text_range_89_90(i);
    } else {
        css_text_range_90_91(i);
    }
}

pub proof fn css_text_range_88_91(i: int)
    requires
        5632 <= i < 5824,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 5696 {
        css_text_range_88_89(i);
    } else {
        css_text_range_89_91(i);
    }
}

pub proof fn css_text_range_86_91(i: int)
    requires
        5504 <= i < 5824,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 5632 {
        css_text_range_86_88(i);
    } else {
        css_text_range_88_91(i);
    }
}

pub proof fn css_text_range_81_91(i: int)
    requires
        5184 <= i < 5824,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 5504 {
        css_text_range_81_86(i);
    } else {
        css_text_range_86_91(i);
    }
}

pub proof fn css_text_range_91_92(i: int)
    requires
        5824 <= i < 5888,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block91(i);
}

pub proof fn css_text_range_92_93(i: int)
    requires
        5888 <= i < 5952,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block92(i);
}

pub proof fn css_text_range_91_93(i: int)
    requires
        5824 <= i < 5952,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 5888 {
        css_text_range_91_92(i);
    } else {
        css_text_range_92_93(i);
    }
}

pub proof fn css_text_range_93_94(i: int)
    requires
        5952 <= i < 6016,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block93(i);
}

pub proof fn css_text_range_94_95(i: int)
    requires
        6016 <= i < 6080,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block94(i);
}

pub proof fn css_text_range_95_96(i: int)
    requires
        6080 <= i < 6144,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block95(i);
}

pub proof fn css_text_range_94_96(i: int)
    requires
        6016 <= i < 6144,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6080 {
        css_text_range_94_95(i);
    } else {
        css_text_range_95_96(i);
    }
}

pub proof fn css_text_range_93_96(i: int)
    requires
        5952 <= i < 6144,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6016 {
        css_text_range_93_94(i);
    } else {
        css_text_range_94_96(i);
    }
}

pub proof fn css_text_range_91_96(i: int)
    requires
        5824 <= i < 6144,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 5952 {
        css_text_range_91_93(i);
    } else {
        css_text_range_93_96(i);
    }
}

pub proof fn css_text_range_96_97(i: int)
    requires
        6144 <= i < 6208,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block96(i);
}

pub proof fn css_text_range_97_98(i: int)
    requires
        6208 <= i < 6272,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block97(i);
}

pub proof fn css_text_range_96_98(i: int)
    requires
        6144 <= i < 6272,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6208 {
        css_text_range_96_97(i);
    } else {
        css_text_range_97_98(i);
    }
}

pub proof fn css_text_range_98_99(i: int)
    requires
        6272 <= i < 6336,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block98(i);
}

pub proof fn css_text_range_99_100(i: int)
    requires
        6336 <= i < 6400,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block99(i);
}

pub proof fn css_text_range_100_101(i: int)
    requires
        6400 <= i < 6464,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block100(i);
}

pub proof fn css_text_range_99_101(i: int)
    requires
        6336 <= i < 6464,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6400 {
        css_text_range_99_100(i);
    } else {
        css_text_range_100_101(i);
    }
}

pub proof fn css_text_range_98_101(i: int)
    requires
        6272 <= i < 6464,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6336 {
        css_text_range_98_99(i);
    } else {
        css_text_range_99_101(i);
    }
}

pub proof fn css_text_range_96_101(i: int)
    requires
        6144 <= i < 6464,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6272 {
        css_text_range_96_98(i);
    } else {
        css_text_range_98_101(i);
    }
}

pub proof fn css_text_range_91_101(i: int)
    requires
        5824 <= i < 6464,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6144 {
        css_text_range_91_96(i);
    } else {
        css_text_range_96_101(i);
    }
}

pub proof fn css_text_range_81_101(i: int)
    requires
        5184 <= i < 6464,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 5824 {
        css_text_range_81_91(i);
    } else {
        css_text_range_91_101(i);
    }
}

pub proof fn css_text_range_101_102(i: int)
    requires
        6464 <= i < 6528,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block101(i);
}

pub proof fn css_text_range_102_103(i: int)
    requires
        6528 <= i < 6592,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block102(i);
}

pub proof fn css_text_range_101_103(i: int)
    requires
        6464 <= i < 6592,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6528 {
        css_text_range_101_102(i);
    } else {
        css_text_range_102_103(i);
    }
}

pub proof fn css_text_range_103_104(i: int)
    requires
        6592 <= i < 6656,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block103(i);
}

pub proof fn css_text_range_104_105(i: int)
    requires
        6656 <= i < 6720,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block104(i);
}

pub proof fn css_text_range_105_106(i: int)
    requires
        6720 <= i < 6784,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block105(i);
}

pub proof fn css_text_range_104_106(i: int)
    requires
        6656 <= i < 6784,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6720 {
        css_text_range_104_105(i);
    } else {
        css_text_range_105_106(i);
    }
}

pub proof fn css_text_range_103_106(i: int)
    requires
        6592 <= i < 6784,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6656 {
        css_text_range_103_104(i);
    } else {
        css_text_range_104_106(i);
    }
}

pub proof fn css_text_range_101_106(i: int)
    requires
        6464 <= i < 6784,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6592 {
        css_text_range_101_103(i);
    } else {
        css_text_range_103_106(i);
    }
}

pub proof fn css_text_range_106_107(i: int)
    requires
        6784 <= i < 6848,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block106(i);
}

pub proof fn css_text_range_107_108(i: int)
    requires
        6848 <= i < 6912,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block107(i);
}

pub proof fn css_text_range_106_108(i: int)
    requires
        6784 <= i < 6912,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6848 {
        css_text_range_106_107(i);
    } else {
        css_text_range_107_108(i);
    }
}

pub proof fn css_text_range_108_109(i: int)
    requires
        6912 <= i < 6976,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block108(i);
}

pub proof fn css_text_range_109_110(i: int)
    requires
        6976 <= i < 7040,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block109(i);
}

pub proof fn css_text_range_110_111(i: int)
    requires
        7040 <= i < 7104,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block110(i);
}

pub proof fn css_text_range_109_111(i: int)
    requires
        6976 <= i < 7104,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 7040 {
        css_text_range_109_110(i);
    } else {
        css_text_range_110_111(i);
    }
}

pub proof fn css_text_range_108_111(i: int)
    requires
        6912 <= i < 7104,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6976 {
        css_text_range_108_109(i);
    } else {
        css_text_range_109_111(i);
    }
}

pub proof fn css_text_range_106_111(i: int)
    requires
        6784 <= i < 7104,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6912 {
        css_text_range_106_108(i);
    } else {
        css_text_range_108_111(i);
    }
}

pub proof fn css_text_range_101_111(i: int)
    requires
        6464 <= i < 7104,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6784 {
        css_text_range_101_106(i);
    } else {
        css_text_range_106_111(i);
    }
}

pub proof fn css_text_range_111_112(i: int)
    requires
        7104 <= i < 7168,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block111(i);
}

pub proof fn css_text_range_112_113(i: int)
    requires
        7168 <= i < 7232,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block112(i);
}

pub proof fn css_text_range_111_113(i: int)
    requires
        7104 <= i < 7232,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 7168 {
        css_text_range_111_112(i);
    } else {
        css_text_range_112_113(i);
    }
}

pub proof fn css_text_range_113_114(i: int)
    requires
        7232 <= i < 7296,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block113(i);
}

pub proof fn css_text_range_114_115(i: int)
    requires
        7296 <= i < 7360,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block114(i);
}

pub proof fn css_text_range_115_116(i: int)
    requires
        7360 <= i < 7424,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block115(i);
}

pub proof fn css_text_range_114_116(i: int)
    requires
        7296 <= i < 7424,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 7360 {
        css_text_range_114_115(i);
    } else {
        css_text_range_115_116(i);
    }
}

pub proof fn css_text_range_113_116(i: int)
    requires
        7232 <= i < 7424,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 7296 {
        css_text_range_113_114(i);
    } else {
        css_text_range_114_116(i);
    }
}

pub proof fn css_text_range_111_116(i: int)
    requires
        7104 <= i < 7424,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 7232 {
        css_text_range_111_113(i);
    } else {
        css_text_range_113_116(i);
    }
}

pub proof fn css_text_range_116_117(i: int)
    requires
        7424 <= i < 7488,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block116(i);
}

pub proof fn css_text_range_117_118(i: int)
    requires
        7488 <= i < 7552,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block117(i);
}

pub proof fn css_text_range_116_118(i: int)
    requires
        7424 <= i < 7552,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 7488 {
        css_text_range_116_117(i);
    } else {
        css_text_range_117_118(i);
    }
}

pub proof fn css_text_range_118_119(i: int)
    requires
        7552 <= i < 7616,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block118(i);
}

pub proof fn css_text_range_119_120(i: int)
    requires
        7616 <= i < 7680,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block119(i);
}

pub proof fn css_text_range_120_121(i: int)
    requires
        7680 <= i < 7744,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block120(i);
}

pub proof fn css_text_range_119_121(i: int)
    requires
        7616 <= i < 7744,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 7680 {
        css_text_range_119_120(i);
    } else {
        css_text_range_120_121(i);
    }
}

pub proof fn css_text_range_118_121(i: int)
    requires
        7552 <= i < 7744,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 7616 {
        css_text_range_118_119(i);
    } else {
        css_text_range_119_121(i);
    }
}

pub proof fn css_text_range_116_121(i: int)
    requires
        7424 <= i < 7744,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 7552 {
        css_text_range_116_118(i);
    } else {
        css_text_range_118_121(i);
    }
}

pub proof fn css_text_range_111_121(i: int)
    requires
        7104 <= i < 7744,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 7424 {
        css_text_range_111_116(i);
    } else {
        css_text_range_116_121(i);
    }
}

pub proof fn css_text_range_101_121(i: int)
    requires
        6464 <= i < 7744,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 7104 {
        css_text_range_101_111(i);
    } else {
        css_text_range_111_121(i);
    }
}

pub proof fn css_text_range_81_121(i: int)
    requires
        5184 <= i < 7744,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 6464 {
        css_text_range_81_101(i);
    } else {
        css_text_range_101_121(i);
    }
}

pub proof fn css_text_range_121_122(i: int)
    requires
        7744 <= i < 7808,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block121(i);
}

pub proof fn css_text_range_122_123(i: int)
    requires
        7808 <= i < 7872,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block122(i);
}

pub proof fn css_text_range_121_123(i: int)
    requires
        7744 <= i < 7872,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 7808 {
        css_text_range_121_122(i);
    } else {
        css_text_range_122_123(i);
    }
}

pub proof fn css_text_range_123_124(i: int)
    requires
        7872 <= i < 7936,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block123(i);
}

pub proof fn css_text_range_124_125(i: int)
    requires
        7936 <= i < 8000,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block124(i);
}

pub proof fn css_text_range_125_126(i: int)
    requires
        8000 <= i < 8064,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block125(i);
}

pub proof fn css_text_range_124_126(i: int)
    requires
        7936 <= i < 8064,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8000 {
        css_text_range_124_125(i);
    } else {
        css_text_range_125_126(i);
    }
}

pub proof fn css_text_range_123_126(i: int)
    requires
        7872 <= i < 8064,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 7936 {
        css_text_range_123_124(i);
    } else {
        css_text_range_124_126(i);
    }
}

pub proof fn css_text_range_121_126(i: int)
    requires
        7744 <= i < 8064,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 7872 {
        css_text_range_121_123(i);
    } else {
        css_text_range_123_126(i);
    }
}

pub proof fn css_text_range_126_127(i: int)
    requires
        8064 <= i < 8128,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block126(i);
}

pub proof fn css_text_range_127_128(i: int)
    requires
        8128 <= i < 8192,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block127(i);
}

pub proof fn css_text_range_126_128(i: int)
    requires
        8064 <= i < 8192,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8128 {
        css_text_range_126_127(i);
    } else {
        css_text_range_127_128(i);
    }
}

pub proof fn css_text_range_128_129(i: int)
    requires
        8192 <= i < 8256,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block128(i);
}

pub proof fn css_text_range_129_130(i: int)
    requires
        8256 <= i < 8320,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block129(i);
}

pub proof fn css_text_range_130_131(i: int)
    requires
        8320 <= i < 8384,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block130(i);
}

pub proof fn css_text_range_129_131(i: int)
    requires
        8256 <= i < 8384,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8320 {
        css_text_range_129_130(i);
    } else {
        css_text_range_130_131(i);
    }
}

pub proof fn css_text_range_128_131(i: int)
    requires
        8192 <= i < 8384,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8256 {
        css_text_range_128_129(i);
    } else {
        css_text_range_129_131(i);
    }
}

pub proof fn css_text_range_126_131(i: int)
    requires
        8064 <= i < 8384,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8192 {
        css_text_range_126_128(i);
    } else {
        css_text_range_128_131(i);
    }
}

pub proof fn css_text_range_121_131(i: int)
    requires
        7744 <= i < 8384,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8064 {
        css_text_range_121_126(i);
    } else {
        css_text_range_126_131(i);
    }
}

pub proof fn css_text_range_131_132(i: int)
    requires
        8384 <= i < 8448,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block131(i);
}

pub proof fn css_text_range_132_133(i: int)
    requires
        8448 <= i < 8512,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block132(i);
}

pub proof fn css_text_range_131_133(i: int)
    requires
        8384 <= i < 8512,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8448 {
        css_text_range_131_132(i);
    } else {
        css_text_range_132_133(i);
    }
}

pub proof fn css_text_range_133_134(i: int)
    requires
        8512 <= i < 8576,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block133(i);
}

pub proof fn css_text_range_134_135(i: int)
    requires
        8576 <= i < 8640,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block134(i);
}

pub proof fn css_text_range_135_136(i: int)
    requires
        8640 <= i < 8704,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block135(i);
}

pub proof fn css_text_range_134_136(i: int)
    requires
        8576 <= i < 8704,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8640 {
        css_text_range_134_135(i);
    } else {
        css_text_range_135_136(i);
    }
}

pub proof fn css_text_range_133_136(i: int)
    requires
        8512 <= i < 8704,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8576 {
        css_text_range_133_134(i);
    } else {
        css_text_range_134_136(i);
    }
}

pub proof fn css_text_range_131_136(i: int)
    requires
        8384 <= i < 8704,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8512 {
        css_text_range_131_133(i);
    } else {
        css_text_range_133_136(i);
    }
}

pub proof fn css_text_range_136_137(i: int)
    requires
        8704 <= i < 8768,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block136(i);
}

pub proof fn css_text_range_137_138(i: int)
    requires
        8768 <= i < 8832,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block137(i);
}

pub proof fn css_text_range_136_138(i: int)
    requires
        8704 <= i < 8832,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8768 {
        css_text_range_136_137(i);
    } else {
        css_text_range_137_138(i);
    }
}

pub proof fn css_text_range_138_139(i: int)
    requires
        8832 <= i < 8896,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block138(i);
}

pub proof fn css_text_range_139_140(i: int)
    requires
        8896 <= i < 8960,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block139(i);
}

pub proof fn css_text_range_140_141(i: int)
    requires
        8960 <= i < 9024,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block140(i);
}

pub proof fn css_text_range_139_141(i: int)
    requires
        8896 <= i < 9024,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8960 {
        css_text_range_139_140(i);
    } else {
        css_text_range_140_141(i);
    }
}

pub proof fn css_text_range_138_141(i: int)
    requires
        8832 <= i < 9024,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8896 {
        css_text_range_138_139(i);
    } else {
        css_text_range_139_141(i);
    }
}

pub proof fn css_text_range_136_141(i: int)
    requires
        8704 <= i < 9024,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8832 {
        css_text_range_136_138(i);
    } else {
        css_text_range_138_141(i);
    }
}

pub proof fn css_text_range_131_141(i: int)
    requires
        8384 <= i < 9024,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8704 {
        css_text_range_131_136(i);
    } else {
        css_text_range_136_141(i);
    }
}

pub proof fn css_text_range_121_141(i: int)
    requires
        7744 <= i < 9024,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 8384 {
        css_text_range_121_131(i);
    } else {
        css_text_range_131_141(i);
    }
}

pub proof fn css_text_range_141_142(i: int)
    requires
        9024 <= i < 9088,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block141(i);
}

pub proof fn css_text_range_142_143(i: int)
    requires
        9088 <= i < 9152,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block142(i);
}

pub proof fn css_text_range_141_143(i: int)
    requires
        9024 <= i < 9152,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9088 {
        css_text_range_141_142(i);
    } else {
        css_text_range_142_143(i);
    }
}

pub proof fn css_text_range_143_144(i: int)
    requires
        9152 <= i < 9216,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block143(i);
}

pub proof fn css_text_range_144_145(i: int)
    requires
        9216 <= i < 9280,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block144(i);
}

pub proof fn css_text_range_145_146(i: int)
    requires
        9280 <= i < 9344,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block145(i);
}

pub proof fn css_text_range_144_146(i: int)
    requires
        9216 <= i < 9344,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9280 {
        css_text_range_144_145(i);
    } else {
        css_text_range_145_146(i);
    }
}

pub proof fn css_text_range_143_146(i: int)
    requires
        9152 <= i < 9344,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9216 {
        css_text_range_143_144(i);
    } else {
        css_text_range_144_146(i);
    }
}

pub proof fn css_text_range_141_146(i: int)
    requires
        9024 <= i < 9344,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9152 {
        css_text_range_141_143(i);
    } else {
        css_text_range_143_146(i);
    }
}

pub proof fn css_text_range_146_147(i: int)
    requires
        9344 <= i < 9408,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block146(i);
}

pub proof fn css_text_range_147_148(i: int)
    requires
        9408 <= i < 9472,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block147(i);
}

pub proof fn css_text_range_146_148(i: int)
    requires
        9344 <= i < 9472,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9408 {
        css_text_range_146_147(i);
    } else {
        css_text_range_147_148(i);
    }
}

pub proof fn css_text_range_148_149(i: int)
    requires
        9472 <= i < 9536,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block148(i);
}

pub proof fn css_text_range_149_150(i: int)
    requires
        9536 <= i < 9600,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block149(i);
}

pub proof fn css_text_range_150_151(i: int)
    requires
        9600 <= i < 9664,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block150(i);
}

pub proof fn css_text_range_149_151(i: int)
    requires
        9536 <= i < 9664,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9600 {
        css_text_range_149_150(i);
    } else {
        css_text_range_150_151(i);
    }
}

pub proof fn css_text_range_148_151(i: int)
    requires
        9472 <= i < 9664,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9536 {
        css_text_range_148_149(i);
    } else {
        css_text_range_149_151(i);
    }
}

pub proof fn css_text_range_146_151(i: int)
    requires
        9344 <= i < 9664,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9472 {
        css_text_range_146_148(i);
    } else {
        css_text_range_148_151(i);
    }
}

pub proof fn css_text_range_141_151(i: int)
    requires
        9024 <= i < 9664,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9344 {
        css_text_range_141_146(i);
    } else {
        css_text_range_146_151(i);
    }
}

pub proof fn css_text_range_151_152(i: int)
    requires
        9664 <= i < 9728,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block151(i);
}

pub proof fn css_text_range_152_153(i: int)
    requires
        9728 <= i < 9792,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block152(i);
}

pub proof fn css_text_range_151_153(i: int)
    requires
        9664 <= i < 9792,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9728 {
        css_text_range_151_152(i);
    } else {
        css_text_range_152_153(i);
    }
}

pub proof fn css_text_range_153_154(i: int)
    requires
        9792 <= i < 9856,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block153(i);
}

pub proof fn css_text_range_154_155(i: int)
    requires
        9856 <= i < 9920,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block154(i);
}

pub proof fn css_text_range_155_156(i: int)
    requires
        9920 <= i < 9984,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block155(i);
}

pub proof fn css_text_range_154_156(i: int)
    requires
        9856 <= i < 9984,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9920 {
        css_text_range_154_155(i);
    } else {
        css_text_range_155_156(i);
    }
}

pub proof fn css_text_range_153_156(i: int)
    requires
        9792 <= i < 9984,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9856 {
        css_text_range_153_154(i);
    } else {
        css_text_range_154_156(i);
    }
}

pub proof fn css_text_range_151_156(i: int)
    requires
        9664 <= i < 9984,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9792 {
        css_text_range_151_153(i);
    } else {
        css_text_range_153_156(i);
    }
}

pub proof fn css_text_range_156_157(i: int)
    requires
        9984 <= i < 10048,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block156(i);
}

pub proof fn css_text_range_157_158(i: int)
    requires
        10048 <= i < 10112,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block157(i);
}

pub proof fn css_text_range_158_159(i: int)
    requires
        10112 <= i < 10176,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block158(i);
}

pub proof fn css_text_range_157_159(i: int)
    requires
        10048 <= i < 10176,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 10112 {
        css_text_range_157_158(i);
    } else {
        css_text_range_158_159(i);
    }
}

pub proof fn css_text_range_156_159(i: int)
    requires
        9984 <= i < 10176,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 10048 {
        css_text_range_156_157(i);
    } else {
        css_text_range_157_159(i);
    }
}

pub proof fn css_text_range_159_160(i: int)
    requires
        10176 <= i < 10240,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block159(i);
}

pub proof fn css_text_range_160_161(i: int)
    requires
        10240 <= i < 10304,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block160(i);
}

pub proof fn css_text_range_161_162(i: int)
    requires
        10304 <= i < 10326,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    css_text_block161(i);
}

pub proof fn css_text_range_160_162(i: int)
    requires
        10240 <= i < 10326,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 10304 {
        css_text_range_160_161(i);
    } else {
        css_text_range_161_162(i);
    }
}

pub proof fn css_text_range_159_162(i: int)
    requires
        10176 <= i < 10326,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 10240 {
        css_text_range_159_160(i);
    } else {
        css_text_range_160_162(i);
    }
}

pub proof fn css_text_range_156_162(i: int)
    requires
        9984 <= i < 10326,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 10176 {
        css_text_range_156_159(i);
    } else {
        css_text_range_159_162(i);
    }
}

pub proof fn css_text_range_151_162(i: int)
    requires
        9664 <= i < 10326,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9984 {
        css_text_range_151_156(i);
    } else {
        css_text_range_156_162(i);
    }
}

pub proof fn css_text_range_141_162(i: int)
    requires
        9024 <= i < 10326,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9664 {
        css_text_range_141_151(i);
    } else {
        css_text_range_151_162(i);
    }
}

pub proof fn css_text_range_121_162(i: int)
    requires
        7744 <= i < 10326,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 9024 {
        css_text_range_121_141(i);
    } else {
        css_text_range_141_162(i);
    }
}

pub proof fn css_text_range_81_162(i: int)
    requires
        5184 <= i < 10326,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 7744 {
        css_text_range_81_121(i);
    } else {
        css_text_range_121_162(i);
    }
}

pub proof fn css_text_range_0_162(i: int)
    requires
        0 <= i < 10326,
    ensures
        (css_text_chars()[i] as u8) == ch::css_text_bytes()[i],
        '\x00' <= css_text_chars()[i] <= '\x7f',
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    if i < 5184 {
        css_text_range_0_81(i);
    } else {
        css_text_range_81_162(i);
    }
}

pub proof fn css_text_shape()
    ensures
        css_text_chars().len() == 10326,
        ch::css_text_bytes().len() == 10326,
        u::css_text() == encode_utf8(css_text_chars()),
{
    hide(ch::css_text_bytes);
    reveal_css_text!();
    assert(ch::css_text_bytes().len() == 10326) by (compute_only);
}

pub proof fn css_text_encoding()
    ensures
        u::css_text() == ch::css_text_bytes(),
{
    hide(css_text_chars);
    hide(ch::css_text_bytes);
    hide(u::css_text);
    css_text_shape();
    assert forall|i: int| 0 <= i < css_text_chars().len() implies '\x00'
        <= #[trigger] css_text_chars()[i] <= '\x7f' by {
        css_text_range_0_162(i);
    }
    is_ascii_chars_encode_utf8(css_text_chars());
    assert forall|i: int| 0 <= i < 10326 implies #[trigger] encode_utf8(css_text_chars())[i]
        == ch::css_text_bytes()[i] by {
        css_text_range_0_162(i);
    }
    assert(encode_utf8(css_text_chars()) =~= ch::css_text_bytes());
}

pub open spec fn script_html_chars() -> Seq<char> {
    r##"<script>
(function () {
"use strict";
var picked = "";
function groupOf(node) {
if (node === null) { return ""; }
var m = node.closest("mark");
if (m === null) { return ""; }
var name = m.classList.item(0);
if (name === null) { return ""; }
return name;
}
function clearPick() {
if (picked === "") { return; }
document.getElementById("main").classList.remove("hl-click");
var marks = document.querySelectorAll("mark.hl-pick");
var i = 0;
while (i < marks.length) { marks[i].classList.remove("hl-pick"); i += 1; }
picked = "";
}
function toggleBox() { return document.querySelector("input.hl-toggle"); }
document.addEventListener("click", function (ev) {
var name = groupOf(ev.target);
if (name === "") { return; }
var box = toggleBox();
if (box === null) { return; }
if (box.checked === false) { return; }
if (picked === name) { clearPick(); return; }
clearPick();
var marks = document.querySelectorAll("mark." + name);
var i = 0;
while (i < marks.length) { marks[i].classList.add("hl-pick"); i += 1; }
document.getElementById("main").classList.add("hl-click");
picked = name;
});
document.addEventListener("mouseout", function (ev) {
if (picked === "") { return; }
if (groupOf(ev.target) !== picked) { return; }
if (groupOf(ev.relatedTarget) === picked) { return; }
clearPick();
});
document.addEventListener("change", function (ev) {
var box = toggleBox();
if (box === null) { return; }
if (ev.target === box) { if (box.checked === false) { clearPick(); } }
});
})();
</script>"##@
}

pub proof fn script_html_block0(i: int)
    requires
        0 <= i < 64,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(0, 64) == seq![
        60u8,
        115,
        99,
        114,
        105,
        112,
        116,
        62,
        10,
        40,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        32,
        40,
        41,
        32,
        123,
        10,
        34,
        117,
        115,
        101,
        32,
        115,
        116,
        114,
        105,
        99,
        116,
        34,
        59,
        10,
        118,
        97,
        114,
        32,
        112,
        105,
        99,
        107,
        101,
        100,
        32,
        61,
        32,
        34,
        34,
        59,
        10,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        32,
    ]) by (compute_only);
}

pub proof fn script_html_block1(i: int)
    requires
        64 <= i < 128,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(64, 128) == seq![
        103u8,
        114,
        111,
        117,
        112,
        79,
        102,
        40,
        110,
        111,
        100,
        101,
        41,
        32,
        123,
        10,
        105,
        102,
        32,
        40,
        110,
        111,
        100,
        101,
        32,
        61,
        61,
        61,
        32,
        110,
        117,
        108,
        108,
        41,
        32,
        123,
        32,
        114,
        101,
        116,
        117,
        114,
        110,
        32,
        34,
        34,
        59,
        32,
        125,
        10,
        118,
        97,
        114,
        32,
        109,
        32,
        61,
        32,
        110,
        111,
        100,
        101,
        46,
        99,
    ]) by (compute_only);
}

pub proof fn script_html_block2(i: int)
    requires
        128 <= i < 192,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(128, 192) == seq![
        108u8,
        111,
        115,
        101,
        115,
        116,
        40,
        34,
        109,
        97,
        114,
        107,
        34,
        41,
        59,
        10,
        105,
        102,
        32,
        40,
        109,
        32,
        61,
        61,
        61,
        32,
        110,
        117,
        108,
        108,
        41,
        32,
        123,
        32,
        114,
        101,
        116,
        117,
        114,
        110,
        32,
        34,
        34,
        59,
        32,
        125,
        10,
        118,
        97,
        114,
        32,
        110,
        97,
        109,
        101,
        32,
        61,
        32,
        109,
        46,
        99,
        108,
        97,
        115,
    ]) by (compute_only);
}

pub proof fn script_html_block3(i: int)
    requires
        192 <= i < 256,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(192, 256) == seq![
        115u8,
        76,
        105,
        115,
        116,
        46,
        105,
        116,
        101,
        109,
        40,
        48,
        41,
        59,
        10,
        105,
        102,
        32,
        40,
        110,
        97,
        109,
        101,
        32,
        61,
        61,
        61,
        32,
        110,
        117,
        108,
        108,
        41,
        32,
        123,
        32,
        114,
        101,
        116,
        117,
        114,
        110,
        32,
        34,
        34,
        59,
        32,
        125,
        10,
        114,
        101,
        116,
        117,
        114,
        110,
        32,
        110,
        97,
        109,
        101,
        59,
        10,
        125,
        10,
    ]) by (compute_only);
}

pub proof fn script_html_block4(i: int)
    requires
        256 <= i < 320,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(256, 320) == seq![
        102u8,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        32,
        99,
        108,
        101,
        97,
        114,
        80,
        105,
        99,
        107,
        40,
        41,
        32,
        123,
        10,
        105,
        102,
        32,
        40,
        112,
        105,
        99,
        107,
        101,
        100,
        32,
        61,
        61,
        61,
        32,
        34,
        34,
        41,
        32,
        123,
        32,
        114,
        101,
        116,
        117,
        114,
        110,
        59,
        32,
        125,
        10,
        100,
        111,
        99,
        117,
        109,
        101,
        110,
        116,
        46,
        103,
    ]) by (compute_only);
}

pub proof fn script_html_block5(i: int)
    requires
        320 <= i < 384,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(320, 384) == seq![
        101u8,
        116,
        69,
        108,
        101,
        109,
        101,
        110,
        116,
        66,
        121,
        73,
        100,
        40,
        34,
        109,
        97,
        105,
        110,
        34,
        41,
        46,
        99,
        108,
        97,
        115,
        115,
        76,
        105,
        115,
        116,
        46,
        114,
        101,
        109,
        111,
        118,
        101,
        40,
        34,
        104,
        108,
        45,
        99,
        108,
        105,
        99,
        107,
        34,
        41,
        59,
        10,
        118,
        97,
        114,
        32,
        109,
        97,
        114,
        107,
        115,
        32,
        61,
        32,
    ]) by (compute_only);
}

pub proof fn script_html_block6(i: int)
    requires
        384 <= i < 448,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(384, 448) == seq![
        100u8,
        111,
        99,
        117,
        109,
        101,
        110,
        116,
        46,
        113,
        117,
        101,
        114,
        121,
        83,
        101,
        108,
        101,
        99,
        116,
        111,
        114,
        65,
        108,
        108,
        40,
        34,
        109,
        97,
        114,
        107,
        46,
        104,
        108,
        45,
        112,
        105,
        99,
        107,
        34,
        41,
        59,
        10,
        118,
        97,
        114,
        32,
        105,
        32,
        61,
        32,
        48,
        59,
        10,
        119,
        104,
        105,
        108,
        101,
        32,
        40,
        105,
        32,
        60,
    ]) by (compute_only);
}

pub proof fn script_html_block7(i: int)
    requires
        448 <= i < 512,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(448, 512) == seq![
        32u8,
        109,
        97,
        114,
        107,
        115,
        46,
        108,
        101,
        110,
        103,
        116,
        104,
        41,
        32,
        123,
        32,
        109,
        97,
        114,
        107,
        115,
        91,
        105,
        93,
        46,
        99,
        108,
        97,
        115,
        115,
        76,
        105,
        115,
        116,
        46,
        114,
        101,
        109,
        111,
        118,
        101,
        40,
        34,
        104,
        108,
        45,
        112,
        105,
        99,
        107,
        34,
        41,
        59,
        32,
        105,
        32,
        43,
        61,
        32,
        49,
        59,
        32,
        125,
    ]) by (compute_only);
}

pub proof fn script_html_block8(i: int)
    requires
        512 <= i < 576,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(512, 576) == seq![
        10u8,
        112,
        105,
        99,
        107,
        101,
        100,
        32,
        61,
        32,
        34,
        34,
        59,
        10,
        125,
        10,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        32,
        116,
        111,
        103,
        103,
        108,
        101,
        66,
        111,
        120,
        40,
        41,
        32,
        123,
        32,
        114,
        101,
        116,
        117,
        114,
        110,
        32,
        100,
        111,
        99,
        117,
        109,
        101,
        110,
        116,
        46,
        113,
        117,
        101,
        114,
        121,
        83,
        101,
        108,
        101,
    ]) by (compute_only);
}

pub proof fn script_html_block9(i: int)
    requires
        576 <= i < 640,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(576, 640) == seq![
        99u8,
        116,
        111,
        114,
        40,
        34,
        105,
        110,
        112,
        117,
        116,
        46,
        104,
        108,
        45,
        116,
        111,
        103,
        103,
        108,
        101,
        34,
        41,
        59,
        32,
        125,
        10,
        100,
        111,
        99,
        117,
        109,
        101,
        110,
        116,
        46,
        97,
        100,
        100,
        69,
        118,
        101,
        110,
        116,
        76,
        105,
        115,
        116,
        101,
        110,
        101,
        114,
        40,
        34,
        99,
        108,
        105,
        99,
        107,
        34,
        44,
        32,
        102,
        117,
    ]) by (compute_only);
}

pub proof fn script_html_block10(i: int)
    requires
        640 <= i < 704,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(640, 704) == seq![
        110u8,
        99,
        116,
        105,
        111,
        110,
        32,
        40,
        101,
        118,
        41,
        32,
        123,
        10,
        118,
        97,
        114,
        32,
        110,
        97,
        109,
        101,
        32,
        61,
        32,
        103,
        114,
        111,
        117,
        112,
        79,
        102,
        40,
        101,
        118,
        46,
        116,
        97,
        114,
        103,
        101,
        116,
        41,
        59,
        10,
        105,
        102,
        32,
        40,
        110,
        97,
        109,
        101,
        32,
        61,
        61,
        61,
        32,
        34,
        34,
        41,
        32,
        123,
        32,
    ]) by (compute_only);
}

pub proof fn script_html_block11(i: int)
    requires
        704 <= i < 768,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(704, 768) == seq![
        114u8,
        101,
        116,
        117,
        114,
        110,
        59,
        32,
        125,
        10,
        118,
        97,
        114,
        32,
        98,
        111,
        120,
        32,
        61,
        32,
        116,
        111,
        103,
        103,
        108,
        101,
        66,
        111,
        120,
        40,
        41,
        59,
        10,
        105,
        102,
        32,
        40,
        98,
        111,
        120,
        32,
        61,
        61,
        61,
        32,
        110,
        117,
        108,
        108,
        41,
        32,
        123,
        32,
        114,
        101,
        116,
        117,
        114,
        110,
        59,
        32,
        125,
        10,
        105,
    ]) by (compute_only);
}

pub proof fn script_html_block12(i: int)
    requires
        768 <= i < 832,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(768, 832) == seq![
        102u8,
        32,
        40,
        98,
        111,
        120,
        46,
        99,
        104,
        101,
        99,
        107,
        101,
        100,
        32,
        61,
        61,
        61,
        32,
        102,
        97,
        108,
        115,
        101,
        41,
        32,
        123,
        32,
        114,
        101,
        116,
        117,
        114,
        110,
        59,
        32,
        125,
        10,
        105,
        102,
        32,
        40,
        112,
        105,
        99,
        107,
        101,
        100,
        32,
        61,
        61,
        61,
        32,
        110,
        97,
        109,
        101,
        41,
        32,
        123,
        32,
        99,
        108,
        101,
    ]) by (compute_only);
}

pub proof fn script_html_block13(i: int)
    requires
        832 <= i < 896,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(832, 896) == seq![
        97u8,
        114,
        80,
        105,
        99,
        107,
        40,
        41,
        59,
        32,
        114,
        101,
        116,
        117,
        114,
        110,
        59,
        32,
        125,
        10,
        99,
        108,
        101,
        97,
        114,
        80,
        105,
        99,
        107,
        40,
        41,
        59,
        10,
        118,
        97,
        114,
        32,
        109,
        97,
        114,
        107,
        115,
        32,
        61,
        32,
        100,
        111,
        99,
        117,
        109,
        101,
        110,
        116,
        46,
        113,
        117,
        101,
        114,
        121,
        83,
        101,
        108,
        101,
        99,
    ]) by (compute_only);
}

pub proof fn script_html_block14(i: int)
    requires
        896 <= i < 960,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(896, 960) == seq![
        116u8,
        111,
        114,
        65,
        108,
        108,
        40,
        34,
        109,
        97,
        114,
        107,
        46,
        34,
        32,
        43,
        32,
        110,
        97,
        109,
        101,
        41,
        59,
        10,
        118,
        97,
        114,
        32,
        105,
        32,
        61,
        32,
        48,
        59,
        10,
        119,
        104,
        105,
        108,
        101,
        32,
        40,
        105,
        32,
        60,
        32,
        109,
        97,
        114,
        107,
        115,
        46,
        108,
        101,
        110,
        103,
        116,
        104,
        41,
        32,
        123,
        32,
        109,
        97,
    ]) by (compute_only);
}

pub proof fn script_html_block15(i: int)
    requires
        960 <= i < 1024,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(960, 1024) == seq![
        114u8,
        107,
        115,
        91,
        105,
        93,
        46,
        99,
        108,
        97,
        115,
        115,
        76,
        105,
        115,
        116,
        46,
        97,
        100,
        100,
        40,
        34,
        104,
        108,
        45,
        112,
        105,
        99,
        107,
        34,
        41,
        59,
        32,
        105,
        32,
        43,
        61,
        32,
        49,
        59,
        32,
        125,
        10,
        100,
        111,
        99,
        117,
        109,
        101,
        110,
        116,
        46,
        103,
        101,
        116,
        69,
        108,
        101,
        109,
        101,
        110,
        116,
        66,
        121,
    ]) by (compute_only);
}

pub proof fn script_html_block16(i: int)
    requires
        1024 <= i < 1088,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(1024, 1088) == seq![
        73u8,
        100,
        40,
        34,
        109,
        97,
        105,
        110,
        34,
        41,
        46,
        99,
        108,
        97,
        115,
        115,
        76,
        105,
        115,
        116,
        46,
        97,
        100,
        100,
        40,
        34,
        104,
        108,
        45,
        99,
        108,
        105,
        99,
        107,
        34,
        41,
        59,
        10,
        112,
        105,
        99,
        107,
        101,
        100,
        32,
        61,
        32,
        110,
        97,
        109,
        101,
        59,
        10,
        125,
        41,
        59,
        10,
        100,
        111,
        99,
        117,
        109,
        101,
        110,
    ]) by (compute_only);
}

pub proof fn script_html_block17(i: int)
    requires
        1088 <= i < 1152,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(1088, 1152) == seq![
        116u8,
        46,
        97,
        100,
        100,
        69,
        118,
        101,
        110,
        116,
        76,
        105,
        115,
        116,
        101,
        110,
        101,
        114,
        40,
        34,
        109,
        111,
        117,
        115,
        101,
        111,
        117,
        116,
        34,
        44,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        32,
        40,
        101,
        118,
        41,
        32,
        123,
        10,
        105,
        102,
        32,
        40,
        112,
        105,
        99,
        107,
        101,
        100,
        32,
        61,
        61,
        61,
        32,
        34,
        34,
    ]) by (compute_only);
}

pub proof fn script_html_block18(i: int)
    requires
        1152 <= i < 1216,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(1152, 1216) == seq![
        41u8,
        32,
        123,
        32,
        114,
        101,
        116,
        117,
        114,
        110,
        59,
        32,
        125,
        10,
        105,
        102,
        32,
        40,
        103,
        114,
        111,
        117,
        112,
        79,
        102,
        40,
        101,
        118,
        46,
        116,
        97,
        114,
        103,
        101,
        116,
        41,
        32,
        33,
        61,
        61,
        32,
        112,
        105,
        99,
        107,
        101,
        100,
        41,
        32,
        123,
        32,
        114,
        101,
        116,
        117,
        114,
        110,
        59,
        32,
        125,
        10,
        105,
        102,
        32,
    ]) by (compute_only);
}

pub proof fn script_html_block19(i: int)
    requires
        1216 <= i < 1280,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(1216, 1280) == seq![
        40u8,
        103,
        114,
        111,
        117,
        112,
        79,
        102,
        40,
        101,
        118,
        46,
        114,
        101,
        108,
        97,
        116,
        101,
        100,
        84,
        97,
        114,
        103,
        101,
        116,
        41,
        32,
        61,
        61,
        61,
        32,
        112,
        105,
        99,
        107,
        101,
        100,
        41,
        32,
        123,
        32,
        114,
        101,
        116,
        117,
        114,
        110,
        59,
        32,
        125,
        10,
        99,
        108,
        101,
        97,
        114,
        80,
        105,
        99,
        107,
        40,
        41,
        59,
        10,
    ]) by (compute_only);
}

pub proof fn script_html_block20(i: int)
    requires
        1280 <= i < 1344,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(1280, 1344) == seq![
        125u8,
        41,
        59,
        10,
        100,
        111,
        99,
        117,
        109,
        101,
        110,
        116,
        46,
        97,
        100,
        100,
        69,
        118,
        101,
        110,
        116,
        76,
        105,
        115,
        116,
        101,
        110,
        101,
        114,
        40,
        34,
        99,
        104,
        97,
        110,
        103,
        101,
        34,
        44,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        32,
        40,
        101,
        118,
        41,
        32,
        123,
        10,
        118,
        97,
        114,
        32,
        98,
        111,
        120,
        32,
    ]) by (compute_only);
}

pub proof fn script_html_block21(i: int)
    requires
        1344 <= i < 1408,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(1344, 1408) == seq![
        61u8,
        32,
        116,
        111,
        103,
        103,
        108,
        101,
        66,
        111,
        120,
        40,
        41,
        59,
        10,
        105,
        102,
        32,
        40,
        98,
        111,
        120,
        32,
        61,
        61,
        61,
        32,
        110,
        117,
        108,
        108,
        41,
        32,
        123,
        32,
        114,
        101,
        116,
        117,
        114,
        110,
        59,
        32,
        125,
        10,
        105,
        102,
        32,
        40,
        101,
        118,
        46,
        116,
        97,
        114,
        103,
        101,
        116,
        32,
        61,
        61,
        61,
        32,
        98,
    ]) by (compute_only);
}

pub proof fn script_html_block22(i: int)
    requires
        1408 <= i < 1472,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(1408, 1472) == seq![
        111u8,
        120,
        41,
        32,
        123,
        32,
        105,
        102,
        32,
        40,
        98,
        111,
        120,
        46,
        99,
        104,
        101,
        99,
        107,
        101,
        100,
        32,
        61,
        61,
        61,
        32,
        102,
        97,
        108,
        115,
        101,
        41,
        32,
        123,
        32,
        99,
        108,
        101,
        97,
        114,
        80,
        105,
        99,
        107,
        40,
        41,
        59,
        32,
        125,
        32,
        125,
        10,
        125,
        41,
        59,
        10,
        125,
        41,
        40,
        41,
        59,
        10,
        60,
        47,
    ]) by (compute_only);
}

pub proof fn script_html_block23(i: int)
    requires
        1472 <= i < 1479,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
    assert(ch::script_html_bytes().subrange(1472, 1479) == seq![115u8, 99, 114, 105, 112, 116, 62])
        by (compute_only);
}

pub proof fn script_html_range_0_1(i: int)
    requires
        0 <= i < 64,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block0(i);
}

pub proof fn script_html_range_1_2(i: int)
    requires
        64 <= i < 128,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block1(i);
}

pub proof fn script_html_range_2_3(i: int)
    requires
        128 <= i < 192,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block2(i);
}

pub proof fn script_html_range_1_3(i: int)
    requires
        64 <= i < 192,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 128 {
        script_html_range_1_2(i);
    } else {
        script_html_range_2_3(i);
    }
}

pub proof fn script_html_range_0_3(i: int)
    requires
        0 <= i < 192,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 64 {
        script_html_range_0_1(i);
    } else {
        script_html_range_1_3(i);
    }
}

pub proof fn script_html_range_3_4(i: int)
    requires
        192 <= i < 256,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block3(i);
}

pub proof fn script_html_range_4_5(i: int)
    requires
        256 <= i < 320,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block4(i);
}

pub proof fn script_html_range_5_6(i: int)
    requires
        320 <= i < 384,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block5(i);
}

pub proof fn script_html_range_4_6(i: int)
    requires
        256 <= i < 384,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 320 {
        script_html_range_4_5(i);
    } else {
        script_html_range_5_6(i);
    }
}

pub proof fn script_html_range_3_6(i: int)
    requires
        192 <= i < 384,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 256 {
        script_html_range_3_4(i);
    } else {
        script_html_range_4_6(i);
    }
}

pub proof fn script_html_range_0_6(i: int)
    requires
        0 <= i < 384,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 192 {
        script_html_range_0_3(i);
    } else {
        script_html_range_3_6(i);
    }
}

pub proof fn script_html_range_6_7(i: int)
    requires
        384 <= i < 448,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block6(i);
}

pub proof fn script_html_range_7_8(i: int)
    requires
        448 <= i < 512,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block7(i);
}

pub proof fn script_html_range_8_9(i: int)
    requires
        512 <= i < 576,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block8(i);
}

pub proof fn script_html_range_7_9(i: int)
    requires
        448 <= i < 576,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 512 {
        script_html_range_7_8(i);
    } else {
        script_html_range_8_9(i);
    }
}

pub proof fn script_html_range_6_9(i: int)
    requires
        384 <= i < 576,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 448 {
        script_html_range_6_7(i);
    } else {
        script_html_range_7_9(i);
    }
}

pub proof fn script_html_range_9_10(i: int)
    requires
        576 <= i < 640,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block9(i);
}

pub proof fn script_html_range_10_11(i: int)
    requires
        640 <= i < 704,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block10(i);
}

pub proof fn script_html_range_11_12(i: int)
    requires
        704 <= i < 768,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block11(i);
}

pub proof fn script_html_range_10_12(i: int)
    requires
        640 <= i < 768,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 704 {
        script_html_range_10_11(i);
    } else {
        script_html_range_11_12(i);
    }
}

pub proof fn script_html_range_9_12(i: int)
    requires
        576 <= i < 768,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 640 {
        script_html_range_9_10(i);
    } else {
        script_html_range_10_12(i);
    }
}

pub proof fn script_html_range_6_12(i: int)
    requires
        384 <= i < 768,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 576 {
        script_html_range_6_9(i);
    } else {
        script_html_range_9_12(i);
    }
}

pub proof fn script_html_range_0_12(i: int)
    requires
        0 <= i < 768,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 384 {
        script_html_range_0_6(i);
    } else {
        script_html_range_6_12(i);
    }
}

pub proof fn script_html_range_12_13(i: int)
    requires
        768 <= i < 832,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block12(i);
}

pub proof fn script_html_range_13_14(i: int)
    requires
        832 <= i < 896,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block13(i);
}

pub proof fn script_html_range_14_15(i: int)
    requires
        896 <= i < 960,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block14(i);
}

pub proof fn script_html_range_13_15(i: int)
    requires
        832 <= i < 960,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 896 {
        script_html_range_13_14(i);
    } else {
        script_html_range_14_15(i);
    }
}

pub proof fn script_html_range_12_15(i: int)
    requires
        768 <= i < 960,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 832 {
        script_html_range_12_13(i);
    } else {
        script_html_range_13_15(i);
    }
}

pub proof fn script_html_range_15_16(i: int)
    requires
        960 <= i < 1024,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block15(i);
}

pub proof fn script_html_range_16_17(i: int)
    requires
        1024 <= i < 1088,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block16(i);
}

pub proof fn script_html_range_17_18(i: int)
    requires
        1088 <= i < 1152,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block17(i);
}

pub proof fn script_html_range_16_18(i: int)
    requires
        1024 <= i < 1152,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 1088 {
        script_html_range_16_17(i);
    } else {
        script_html_range_17_18(i);
    }
}

pub proof fn script_html_range_15_18(i: int)
    requires
        960 <= i < 1152,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 1024 {
        script_html_range_15_16(i);
    } else {
        script_html_range_16_18(i);
    }
}

pub proof fn script_html_range_12_18(i: int)
    requires
        768 <= i < 1152,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 960 {
        script_html_range_12_15(i);
    } else {
        script_html_range_15_18(i);
    }
}

pub proof fn script_html_range_18_19(i: int)
    requires
        1152 <= i < 1216,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block18(i);
}

pub proof fn script_html_range_19_20(i: int)
    requires
        1216 <= i < 1280,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block19(i);
}

pub proof fn script_html_range_20_21(i: int)
    requires
        1280 <= i < 1344,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block20(i);
}

pub proof fn script_html_range_19_21(i: int)
    requires
        1216 <= i < 1344,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 1280 {
        script_html_range_19_20(i);
    } else {
        script_html_range_20_21(i);
    }
}

pub proof fn script_html_range_18_21(i: int)
    requires
        1152 <= i < 1344,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 1216 {
        script_html_range_18_19(i);
    } else {
        script_html_range_19_21(i);
    }
}

pub proof fn script_html_range_21_22(i: int)
    requires
        1344 <= i < 1408,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block21(i);
}

pub proof fn script_html_range_22_23(i: int)
    requires
        1408 <= i < 1472,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block22(i);
}

pub proof fn script_html_range_23_24(i: int)
    requires
        1472 <= i < 1479,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    script_html_block23(i);
}

pub proof fn script_html_range_22_24(i: int)
    requires
        1408 <= i < 1479,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 1472 {
        script_html_range_22_23(i);
    } else {
        script_html_range_23_24(i);
    }
}

pub proof fn script_html_range_21_24(i: int)
    requires
        1344 <= i < 1479,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 1408 {
        script_html_range_21_22(i);
    } else {
        script_html_range_22_24(i);
    }
}

pub proof fn script_html_range_18_24(i: int)
    requires
        1152 <= i < 1479,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 1344 {
        script_html_range_18_21(i);
    } else {
        script_html_range_21_24(i);
    }
}

pub proof fn script_html_range_12_24(i: int)
    requires
        768 <= i < 1479,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 1152 {
        script_html_range_12_18(i);
    } else {
        script_html_range_18_24(i);
    }
}

pub proof fn script_html_range_0_24(i: int)
    requires
        0 <= i < 1479,
    ensures
        (script_html_chars()[i] as u8) == ch::script_html_bytes()[i],
        '\x00' <= script_html_chars()[i] <= '\x7f',
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    if i < 768 {
        script_html_range_0_12(i);
    } else {
        script_html_range_12_24(i);
    }
}

pub proof fn script_html_shape()
    ensures
        script_html_chars().len() == 1479,
        ch::script_html_bytes().len() == 1479,
        u::script_html() == encode_utf8(script_html_chars()),
{
    hide(ch::script_html_bytes);
    reveal_script_html!();
    assert(ch::script_html_bytes().len() == 1479) by (compute_only);
}

pub proof fn script_html_encoding()
    ensures
        u::script_html() == ch::script_html_bytes(),
{
    hide(script_html_chars);
    hide(ch::script_html_bytes);
    hide(u::script_html);
    script_html_shape();
    assert forall|i: int| 0 <= i < script_html_chars().len() implies '\x00'
        <= #[trigger] script_html_chars()[i] <= '\x7f' by {
        script_html_range_0_24(i);
    }
    is_ascii_chars_encode_utf8(script_html_chars());
    assert forall|i: int| 0 <= i < 1479 implies #[trigger] encode_utf8(script_html_chars())[i]
        == ch::script_html_bytes()[i] by {
        script_html_range_0_24(i);
    }
    assert(encode_utf8(script_html_chars()) =~= ch::script_html_bytes());
}

} // verus!
