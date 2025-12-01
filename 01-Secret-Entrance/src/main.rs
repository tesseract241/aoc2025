fn main() {

    let aoc = AocClient::from_token();
    let p = Problem::new(Year::Y2025, 1);
    let i = aoc.get_input(p).unwrap();




    let result = aoc.submit(p, Level::A, answerA).unwrap();
    println("Submission for level A was {}", result)

    let result = aoc.submit(p, Level::B, answerA).unwrap();
    println("Submission for level B was {}", result)
}
