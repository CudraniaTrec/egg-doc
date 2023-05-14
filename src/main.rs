use egg::*;
fn main() {
    // let's make an e-graph
    let mut egraph: EGraph<SymbolLang, ()> = Default::default();
    let a = egraph.add(SymbolLang::leaf("a"));
    let b = egraph.add(SymbolLang::leaf("b"));
    let _foo = egraph.add(SymbolLang::new("foo", vec![a, b]));

    // rebuild the e-graph since we modified it
    egraph.rebuild();

    // we can make Patterns by parsing, similar to RecExprs
    // names preceded by ? are parsed as Pattern variables and will match anything
    let pat: Pattern<SymbolLang> = "(foo ?x ?x)".parse().unwrap();

    // since we use ?x twice, it must match the same thing,
    // so this search will return nothing
    let matches = pat.search(&egraph);
    assert!(matches.is_empty());

    egraph.union(a, b);
    // recall that rebuild must be called to "see" the effects of adds or unions
    egraph.rebuild();

    // now we can find a match since a = b
    let matches = pat.search(&egraph);
    assert!(!matches.is_empty());
}
