//! Office Math (OMML, ECMA-376 Part 1 §22.1) read with `bharat_math::omml`.

use bharat_wordmodel::math::{Math, MathJc, MathProps};
use bharat_wordmodel::props::Rgb;

use crate::xml::{El, Node};

/// This reader's tree as `bharat-xml`'s, which `bharat_math::omml` reads. Both use the canonical
/// prefixes (`m:`, `w:`, `mc:`).
// shortcut: copies the equation's subtree; goes when the reader reads bharat_xml::El directly.
fn to_craft(e: &El) -> bharat_xml::El {
    let mut out = bharat_xml::El { name: e.name.clone(), attrs: e.attrs.clone(), children: Vec::with_capacity(e.kids.len()) };
    for k in &e.kids {
        out.children.push(match k {
            Node::El(c) => bharat_xml::Node::El(to_craft(c)),
            Node::Text(t) => bharat_xml::Node::Text(t.clone()),
        });
    }
    out
}

/// An `m:oMath` element as an equation (structure plus its source XML).
pub fn read_omath(e: &El, jc: MathJc) -> Math {
    bharat_math::omml::read_omath(&to_craft(e), jc)
}

/// The colour every run of `math` shares, taken off the runs (the run around the equation
/// carries it).
pub fn lift_color(math: &mut Math) -> Option<Rgb> {
    bharat_math::omml::lift_color(math)
}

/// The justification an `m:oMathPara` gives its equations.
pub fn para_jc(para: &El) -> MathJc {
    bharat_math::omml::para_jc(&to_craft(para))
}

/// `m:mathPr` in the settings part.
pub fn read_math_pr(e: &El) -> MathProps {
    bharat_math::omml::read_math_pr(&to_craft(e))
}
