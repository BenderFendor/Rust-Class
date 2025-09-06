// Returns the sqrt of `n`
///
/// Using built-in square root functions is not allowed.
pub fn sqrt(n: u32) -> u32 {
    if n < 2 {
        n;
    }

    let mut x = n;
    let mut y = (x + n / x) / 2;

    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

/// Consumes a sorted list of integers and a query integer. Returns the index of the query integer
///
/// Note that a 3-valued comparison between numbers `a` and `b` can be done easily:
/// ```rust,ignore
/// match a.cmp(&b) {
///    std::cmp::Ordering::Less => { ... },
///    std::cmp::Ordering::Greater => { ... },
///    std::cmp::Ordering::Equal => { ... },
/// }
/// ```
pub fn binary_search(arr: &[i32], query: i32) -> Option<u32> {
    if arr.len() == 0 {
        None;
    }

    let mp = arr.len() / 2;

    match query.cmp(&arr[mp]) {
        std::cmp::Ordering::Less => binary_search(&arr[0..mp], query),
        std::cmp::Ordering::Greater => match binary_search(&arr[mp + 1..], query) {
            Some(n) => Some((mp as u32 + 1 as u32 + n as u32) as u32), // Great coding
            None => None,
        },
        std::cmp::Ordering::Equal => Some(mp as u32),
    }
}

/// Consumes a list of numbers representing daily rainfall. The list may contain -999 signifying
/// the end of data of interest. Returns the average of non-negative values up to the first
/// occurrence of -999 (if it occurs). There may be negative numbers other -999 in the list.
/// Returns None if the average is incomputable.
///
/// example: rainfall([6, 8, -1, 1, -999, 4, 5, 6]) -> Some(5.0)
pub fn rainfall(values: &[i32]) -> Option<f64> {
    // Hmm it works.

    let mut sum = 0.0;
    let mut count = 0;

    println!("Values is {:?}", values);

    for n in values {
        if *n == -999 {
            break;
        }
        if *n >= 0 {
            sum += *n as f64;
            count += 1;
        }
    }

    if count > 0 {
        println!("Some is {:?}", Some(sum / count as f64));
        Some(sum / count as f64);
    } else {
       None;
    }
}
