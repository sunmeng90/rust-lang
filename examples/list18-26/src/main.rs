fn main() {
    match_guard_added_to_pattern();
    match_guard_added_to_pattern_to_test_an_outer_variable();
    match_guard_with_or_pattern();
    
}

// Extra Conditionals with Match Guards

// A match guard is an additional if condition specified after the pattern in a match arm that must also match,
// along with the pattern matching, for that arm to be chosen. Match guards are useful for expressing more
// complex ideas than a pattern alone allows.

fn match_guard_added_to_pattern() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

fn match_guard_added_to_pattern_to_test_an_outer_variable() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // The pattern in the second match arm doesn’t introduce a new variable y that would shadow the outer y,
        // meaning we can use the outer y in the match guard.
        //
        // The match guard if n == y is not a pattern and therefore doesn’t introduce new variables. This y is
        // the outer y rather than a new shadowed y, and we can look for a value that has the same value as the
        // outer y by comparing n to y.
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}

// Combining multiple patterns with a match guard

fn match_guard_with_or_pattern() {
    let x = 4;
    let y = false;
    // The match condition states that the arm only matches if the value of x is equal to 4, 5, or 6 and if y is true.

    match x {
        // the precedence of a match guard in relation to a pattern behaves like this:
        // (4 | 5 | 6) if y => ...
        // 
        // NOT: 4 | 5 | (6 if y) => ...
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
