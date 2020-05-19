pub mod set;
pub mod fuzz;
pub mod defuzz;
pub mod common;

use set::*;
use fuzz::*;
use defuzz::*;
use common::*;


#[test]
fn test_input(
) -> FuzzyResult<()> {
    let input = FuzzySet::new()
        .term("very quiet", vec![(0.0, 1.0), (10.0, 1.0), (20.0, 0.5), (30.0, 0.0)])?
        .term("quiet",      vec![(10.0, 0.0), (20.0, 0.5), (30.0, 1.0), (40.0, 1.0), (50.0, 0.5), (60.0, 0.0)])?
        .term("loud",       vec![(40.0, 0.0), (50.0, 0.5), (60.0, 1.0), (70.0, 1.0), (80.0, 0.5), (90.0, 0.0)])?
        .term("very loud",  vec![(70.0, 0.0), (80.0, 0.5), (90.0, 1.0), (100.0, 1.0)])?;

    assert_eq!(input.call(20.0)?.get("very quiet"), Some(&0.5));
    assert_eq!(input.call(20.0)?.get("quiet"), Some(&0.5));
    assert_eq!(input.call(30.0)?.get("very quiet"), Some(&0.0));
    assert_eq!(input.call(-30.0)?.get("very quiet"), Some(&1.0));
    assert_eq!(input.call(130.0)?.get("very quiet"), Some(&0.0));
    assert_eq!(input.call(130.0)?.get("very loud"), Some(&1.0));
    assert_eq!(input.call(130.0)?.get("loud"), Some(&0.0));
    assert_eq!(input.call(90.0)?.get("loud"), Some(&0.0));
    assert_eq!(input.call(90.0)?.get("very loud"), Some(&1.0));
    assert_eq!(input.call(25.0)?.get("very quiet"), Some(&0.25));

    // input.plot("X", "Y", "scatter.svg");
    Ok(())
}

#[test]
fn test_macros(
) -> () {
    assert_eq!(
        unit!("loudness" => "quiet"; "change" => "keep"),
        FuzzyRule::Unit(
            ("loudness".to_string(), "quiet".to_string()),
            ("change".to_string(), "keep".to_string())
        )
    );
    assert_eq!(
        and!("loudness" => "quiet", "tod" => "morning", "param1" => "param2"; "change" => "vol up"),
        FuzzyRule::And(
            vec![
                ("loudness".to_string(), "quiet".to_string()),
                ("tod".to_string(), "morning".to_string()),
                ("param1".to_string(), "param2".to_string())
            ],
            ("change".to_string(), "vol up".to_string()))
    );
    assert_eq!(
        or!("loudness" => "quiet", "tod" => "morning", "param1" => "param2"; "change" => "vol up"),
        FuzzyRule::Or(
            vec![
                ("loudness".to_string(), "quiet".to_string()),
                ("tod".to_string(), "morning".to_string()),
                ("param1".to_string(), "param2".to_string())
            ],
            ("change".to_string(), "vol up".to_string()))
    );
    assert_eq!(
        and!("param1" => "param2"; "change" => "vol up"),
        FuzzyRule::And(
            vec![
                ("param1".to_string(), "param2".to_string())
            ],
            ("change".to_string(), "vol up".to_string()))
    );
}

#[test]
fn test_fuzzy(
) -> FuzzyResult<()> {
    // Macros needed
    let fuzzer = Fuzzer::new().fuzzify(
        "loudness",
        FuzzySet::new()
            .term("very quiet", vec![(0.0, 1.0), (10.0, 1.0), (20.0, 0.5), (30.0, 0.0)])?
            .term("quiet",      vec![(10.0, 0.0), (20.0, 0.5), (30.0, 1.0), (40.0, 1.0), (50.0, 0.5), (60.0, 0.0)])?
            .term("loud",       vec![(40.0, 0.0), (50.0, 0.5), (60.0, 1.0), (70.0, 1.0), (80.0, 0.5), (90.0, 0.0)])?
            .term("very loud",  vec![(70.0, 0.0), (80.0, 0.5), (90.0, 1.0), (100.0, 1.0)])?
    ).fuzzify(
        "tod",
        FuzzySet::new()
            .term("morning", vec![(1.0, 0.0), (3.0, 0.5), (5.0, 1.0), (7.0, 1.0), (9.0, 0.5), (11.0, 0.0)])?
            .term("noon",    vec![(7.0, 0.0), (9.0, 0.50), (11.0, 1.0), (13.0, 1.0), (15.0, 0.50), (17.0, 0.0)])?
            .term("evening", vec![(13.0, 0.0), (15.0, 0.50), (17.0, 1.0), (19.0, 1.0), (21.0, 0.50), (23.0, 0.0)])?
            .term("night",   vec![(0.0, 1.0), (1.0, 1.0), (3.0, 0.5), (5.0, 0.0), (19.0, 0.0), (21.0, 0.5), (23.0, 1.0)])?
    ).defuzzify(
        "change",
        FuzzySet::new()
            .term("vol down", vec![(0.0, 1.0), (2.0, 1.0), (3.0, 0.5), (4.0, 0.0), (7.0, 0.0)])?
            .term("keep", vec![(2.0, 0.0), (3.0, 0.5), (4.0, 1.0), (6.0, 1.0), (7.0, 0.5), (8.0, 0.0)])?
            .term("vol up", vec![(3.0, 0.0), (6.0, 0.0), (7.0, 0.5), (8.0, 1.0), (10.0, 1.0)])?
    )
    // Good Moaning
        .rule(and!("loudness" => "very quiet", "tod" => "morning"; "change" => "vol up"))
        .rule(and!("loudness" => "quiet", "tod" => "morning"; "change" => "keep"))
        .rule(and!("loudness" => "loud", "tod" => "morning"; "change" => "keep"))
        .rule(and!("loudness" => "very loud", "tod" => "morning"; "change" => "vol down"))
        // Noon
        .rule(and!("loudness" => "very quiet", "tod" => "noon"; "change" => "vol up"))
        .rule(and!("loudness" => "quiet", "tod" => "noon"; "change" => "vol up"))
        .rule(and!("loudness" => "loud", "tod" => "noon"; "change" => "keep"))
        .rule(and!("loudness" => "very loud", "tod" => "noon"; "change" => "vol down"))
        // Evening
        .rule(and!("loudness" => "very quiet", "tod" => "evening"; "change" => "vol up"))
        .rule(and!("loudness" => "quiet", "tod" => "evening"; "change" => "keep"))
        .rule(and!("loudness" => "loud", "tod" => "evening"; "change" => "vol down"))
        .rule(and!("loudness" => "very loud", "tod" => "evening"; "change" => "vol down"))
        // Night
        .rule(and!("loudness" => "very quiet", "tod" => "night"; "change" => "vol up"))
        .rule(and!("loudness" => "quiet", "tod" => "night"; "change" => "keep"))
        .rule(and!("loudness" => "loud", "tod" => "night"; "change" => "vol down"))
        .rule(and!("loudness" => "very loud", "tod" => "night"; "change" => "vol down"));

    let mut values = std::collections::HashMap::new();
    values.insert("loudness".to_string(), 50.0);
    values.insert("tod".to_string(), 12.0);

    let sets = fuzzer.apply(&values).unwrap();
    let change_set = sets.get("change").unwrap();
    change_set.plot("X", "Y", "scatter.svg");
    let cog = defuzz::cog(change_set.points("out")?);
    println!("=============== COG: {:?} ===============", cog) ;
    println!("{:?}", change_set);

    assert!(false);
    Ok(())
}
