/*
 * The formula can simplify like:
 * 1/2 2/3 3/4
 * 1/3 2/4 3/5  ==> 4 + 1/2
 * 1/4 2/5 3/6
 * 
 * 1/2 2/3 3/4 4/5
 * 1/3 2/4 3/5 4/6  ==> 4 + 1/2 + 3 + 1/2
 * 1/4 2/5 3/6 4/7
 * 1/5 2/6 3/7 4/8
 * 
 * 
 * ==>  n = 1      2     3     4
 *          1/2    4/2     9/2   16/2
 */
fn game(n: u64) -> Vec<u64> {
    let total = n * n;
    match (n % 2) {
        0 => vec![total/2],
        _ => vec![total, 2],
    }
}

fn testing(n: u64, exp: Vec<u64>) -> () {
    assert_eq!(game(n), exp)
}

#[test]
fn basics_game() {

    testing(204, vec![20808]);
    testing(807, vec![651249, 2]);
    testing(5014, vec![12570098]);
    testing(750001, vec![562501500001, 2]);

}