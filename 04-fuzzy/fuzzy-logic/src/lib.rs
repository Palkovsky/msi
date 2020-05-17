#![allow(dead_code)]
use std::collections::HashMap;
// Plotlib
use plotlib::{
    page::Page,
    repr::Plot,
    view::ContinuousView,
    style::LineStyle
};

#[derive(Debug, Eq, PartialEq)]
pub enum FuzzyError {
    InvalidPoints,
    InvalidCategory(String),
    InvalidTerm(String),
    Misc(String)
}

type FuzzyResult<T> = Result<T, FuzzyError>;

#[derive(Debug, Clone, PartialEq)]
pub struct FuzzySet {
    terms: HashMap<String, Vec<(f64, f64)>>
}

impl FuzzySet {
    pub fn new(
    ) -> Self {
        Self {
            terms: HashMap::new()
        }
    }

    pub fn term(
        mut self,
        key: impl Into<String>,
        mut points: Vec<(f64, f64)>
    ) -> FuzzyResult<Self> {
        if points.len() < 2 {
            Err(FuzzyError::InvalidPoints)?
        }
        // Order points by x axis.
        points.sort_by(|(x1, _), (x2, _)| x1.partial_cmp(x2).unwrap());
        self.terms.insert(key.into(), points);
        Ok(self)
    }

    pub fn terms(
        &self
    ) -> Vec<String> {
        self.terms.keys().cloned().collect()
    }

    fn points(
        &self,
        term: impl Into<String>
    ) -> FuzzyResult<&Vec<(f64, f64)>> {
        let key = term.into();
        self.terms.get(&key)
            .ok_or(FuzzyError::InvalidTerm(key.into()))
    }

    fn plot(
        &self,
        x_label: impl Into<String>,
        y_label: impl Into<String>,
        out_file: impl AsRef<std::path::Path>
    ) -> () {
        let mut view = ContinuousView::new()
            .y_range(0.0, 1.0)
            .x_label(x_label)
            .y_label(y_label);

        for (_key, points) in self.terms.iter(){
            let style = LineStyle::new().colour("#DD3355");
            let plot = Plot::new(points.to_vec())
                .line_style(style);
            view = view.add(plot);
        }

        Page::single(&view).save(out_file).unwrap();
    }

    fn cog(
        &self,
        term: impl Into<String>
    ) -> FuzzyResult<f64> {
        let key = term.into();
        let points = self.terms.get(&key)
            .
            ok_or(FuzzyError::InvalidTerm(key.clone()))?;

        //              / - - -
        //             /
        //      - - - /
        //    /
        //   /
        //- -
        let (mut a, mut b) = (0.0, 0.0);
        for pair in points.windows(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];

            let (x_diff, y_diff) = (x2-x1, (y2-y1).abs());

            // Height of base rectangle.
            let height = f64::min(y1, y2);
            println!("HEIGHT: {}", height);
            // Area of base rectangle + traingle on top.
            let area = height*x_diff + 0.5*y_diff*x_diff;
            println!("AREA: {}", area);
            let centroid = if y_diff == 0.0 {
                (x1+x2)/2.0
            } else {
                (2.0*x1+x2)/3.0
            };
            println!("CENTR: {}", centroid);
            a += area*centroid;
            b += area;
        }
        Ok(a/b)
    }

    /// Aplies maximum threshold for given term.
    fn apply_threshold(
        &mut self,
        term: impl Into<String>,
        value: f64
    ) -> FuzzyResult<()> {
        let key = term.into();
        let mut points = self.terms.remove(&key)
            .ok_or(FuzzyError::InvalidTerm(key.clone()))?;

        // Three cases:
        // 1. Threshold above maximum y -> Do nothing
        // 2. Threshold exceeded only once. -> Replace interval with one point.
        // 3. Whole set above the threshold -> Replace with two bounadry points with same y.
        // 4. Threshold exceeded and returned to valid range -> Replace interval with two points.

        // Solution idea
        // 1. Find all adjacent points above the threshold. This is going to be our interval.
        // 2. Solve for y=value(boundaries). Result: (x1, y), (x2, y), or (x1, y) for case 2).
        // 3. Replace interval with newly calculated points.

        let points_copy = points.clone();

        let find_x = |i: usize, y: f64| -> FuzzyResult<f64> {
            let p1 = points_copy.get(i).unwrap();
            let p2 = points_copy.get(i+1).unwrap_or_else(|| p1);
            println!("Y: {} | P1: {:?} | P2: {:?}", y, p1, p2);
            let (x1, y1) = p1;
            let (x2, y2) = p2;
            // Check if valid range.
            let (min_y, max_y) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
            if !(y >= *min_y && y <= *max_y) {
                panic!("Internal error. y out of bounds.")
            }
            // I assume (x, y) lies on the same line as p1 and p2, so we can find x by comparing slopes.
            let slope = if x1 == x2  { 0.0 } else { (y1-y2)/(x1-x2) };
            let x = if slope == 0.0 { *x1 } else { (y-y1)/slope+x1 };
            Ok(x)
        };

        // Do actual replacement.
        let mut replace_interval = |from: i32, to: i32| -> FuzzyResult<()> {
            println!("REPLACE: {}/{}", from, to);
            match (from, to) {
                (-1, -1) => {},
                // Whole set above threshold -> Replace whole set with two points.
                (-1, j) | (0, j) if j as usize == points_copy.len()-1 => {
                    let (x1, _) = *points.first().unwrap();
                    let (x2, _) = *points.last().unwrap();
                    points.clear();
                    points.push((x1, value));
                    points.push((x2, value));
                },
                // Right side of the set above threshold -> Replace with one point.
                (i, j) if j as usize == points_copy.len()-1 => {
                    let i = i as usize;
                    let j = j as usize;
                    let x1 = find_x(i-1, value)?;
                    points.drain(j..points_copy.len());
                    points.push((x1, value));
                },
                // Left side of the set above threshold -> Replace with one point
                (0, j) | (-1, j) => {
                    let j = j as usize;
                    let x1 = find_x(j, value)?;
                    points.drain(0..j+1);
                    points.insert(0, (x1, value));
                },
                // Middle of set above threshold -> Replace sliced interval with two points.
                (i, j) => {
                    let (i, j) = (i as usize, j as usize);
                    let x1 = find_x(i-1, value)?;
                    let x2 = find_x(j, value)?;
                    points.drain(i..j+1);
                    points.insert(i, (x2, value));
                    points.insert(i, (x1, value));
                }
            };
            Ok(())
        };

        // Pointers to interval positions.
        let (mut int_start, mut int_end) = (-1, -1);
        // Start from the end to prevent indexing issues after element removal.
        for (i, (_x, y)) in points_copy.iter().enumerate().rev() {
            println!("{}/({}, {})", i, _x, y);
            match (int_start, int_end) {
                // No interval processed.
                (-1, -1) => {
                    if *y >= value {
                        int_end = i as i32;
                        println!("HERE 1 {} -> {}", i, int_end);
                    }
                },
                // Start of the interval
                (-1, _) => {
                    if *y < value {
                        int_start = i as i32+1;
                        println!("HERE 2 {} -> ({}, {})", i, int_start, int_end);
                        replace_interval(int_start, int_end)?;
                        int_start = -1;
                        int_end = -1;
                    }
                },
                _ =>
                    panic!("Internal error. Unexpected interval state.")
            }
        }
        replace_interval(int_start, int_end)?;

        self.terms.insert(key, points);
        Ok(())
    }

    fn call_single(
        &self,
        term: impl Into<String>,
        x: f64
    ) -> FuzzyResult<f64> {
        let key = term.into();
        let values = self.terms.get(&key)
            .ok_or(FuzzyError::InvalidTerm(key))?;

        let values = &values[..];
        let (mut lx, mut ly) = values.first().unwrap();
        let (mut rx, mut ry) = values.last().unwrap();
        if x < lx {
            rx = lx; ry = ly;
        } else if x > rx {
            lx = rx; ly = ry;
        } else {
            for window in values.windows(2) {
                let (px, py) = window[0];
                let (cx, cy) = window[1];
                if px <= x && cx >= x {
                    lx = px; ly = py;
                    rx = cx; ry = cy;
                    break;
                }
            }
        }
        // Linear interpolation to find y.
        let slope = if lx == rx  { 0.0 } else { (ly-ry)/(lx-rx) };
        Ok(ly+(x-lx)*slope)
    }

    fn call(
        &self,
        x: f64
    ) -> FuzzyResult<HashMap<String, f64>> {
        let mut result = HashMap::new();
        for key in self.terms() {
            let y = self.call_single(key.clone(), x)?;
            result.insert(key, y);
        }
        Ok(result)
    }
}

#[derive(Debug)]
struct FuzzySetBuilder<'a> {
    base: &'a FuzzySet,
    values: HashMap<String, f64>
}

impl<'a> FuzzySetBuilder<'a> {
    fn new(
        base: &'a FuzzySet
    ) -> Self {
        Self {
            base, values: HashMap::new()
        }
    }

    fn value(
        &mut self,
        term: impl Into<String>,
        y: f64
    ) -> &mut Self {
        let key = term.into();
        // Accumulation method should be configurable.
        // By default it takes MAX.
        let accum = |old: f64, new: f64| if new > old { new } else { old };
        let next = self.values.get(&key).map(|current| accum(*current, y)).unwrap_or_else(|| y);
        self.values.insert(key, next);
        self
    }

    fn build(
        &self,
        term_name: impl Into<String>
    ) -> FuzzyResult<FuzzySet> {
        let mut set = self.base.clone();

        println!("SET1: {:?}", self);
        // Apply thresholds stored in self.values to output set
        for term in self.base.terms() {
            let thres = self.values.get(&term).unwrap_or_else(|| &0.0);
            println!("{}: {}", term, thres);
            println!("BEFORE: {:?}", set);
            set.apply_threshold(term, *thres)?;
            println!("AFTER: {:?}", set);
        }

        //println!("SET2: {:?}", set.terms);
        let mut xs = self.base.terms
            .iter()
            .map(|(_, points)| points.iter().map(|(x, _)| *x).collect::<Vec<f64>>())
            .flatten()
            .collect::<Vec<f64>>();

        xs.sort_by(|x, y| x.partial_cmp(y).unwrap());
        xs.dedup();

        let mut points = Vec::with_capacity(xs.len());
        for x in xs.iter() {
            let results = set.call(*x)?;
            let y = results.into_iter().fold(0.0, |acc, (_, y)| acc+y);
            points.push((*x, y));
        }

        let out = FuzzySet::new().term(term_name.into(), points)?;
        Ok(out)
    }
}

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

// or!("loudness" => "quiet", "tod" => "morning"; "change" => "vol up")

#[macro_export]
macro_rules! unit {
    ($c1:expr=>$t1:expr; $co:expr=>$to:expr) => {
        $crate::FuzzyRule::Unit(
            (String::from($c1), String::from($t1)),
            (String::from($co), String::from($to)))
    }
}

macro_rules! make_rule {
    ($d: tt $name:ident, $typ: tt) => {
        #[allow(unused_macros)]
        #[macro_export]
        macro_rules! $name {
            ($d($d c1:expr=>$d t1:expr),*;$co:expr=>$to:expr) => {
                $crate::FuzzyRule::$typ(
                    vec![$d((String::from($d c1), String::from($d t1)),)*],
                    (String::from($co), String::from($to))
                )
            }
        }
    }
}
make_rule!($ and, And);
make_rule!($ or, Or);

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

    let mut values = HashMap::new();
    values.insert("loudness".to_string(), 50.0);
    values.insert("tod".to_string(), 12.0);

    let sets = fuzzer.apply(&values).unwrap();
    let change_set = sets.get("change").unwrap();
    change_set.plot("X", "Y", "scatter.svg");
    let cog = change_set.cog("out")?;

    println!("=============== COG: {} ===============", cog) ;
    println!("{:?}", change_set);

    assert!(false);
    Ok(())
}

type Category = String;
type Term = String;
type FuzzyValue = (Category, Term, f64);
type FuzzyIdent = (Category, Term);

#[derive(Debug, PartialEq)]
pub enum FuzzyRule {
    Unit(FuzzyIdent, FuzzyIdent),
    And(Vec<FuzzyIdent>, FuzzyIdent),
    Or(Vec<FuzzyIdent>, FuzzyIdent)
}

impl FuzzyRule {
    fn apply(
        &self,
        fuzzer: &Fuzzer,
        values: &HashMap<Category, f64>
    ) -> FuzzyResult<FuzzyValue> {
        // Extract fuzzy sets identifiers and output category
        let (idents, (out_category, out_term)) = match self {
            FuzzyRule::Unit(ident, out) => (vec![ident.clone()], out.clone()),
            FuzzyRule::And(idents, out) => (idents.clone(), out.clone()),
            FuzzyRule::Or(idents, out) => (idents.clone(), out.clone())
        };

        // Sample fuzzy sets
        let mut ys = Vec::new();
        for (cat, term) in idents {
            let y = values.get(&cat)
                .ok_or(FuzzyError::InvalidCategory(cat.clone()))
                .and_then(|x| fuzzer.call(&(cat, term.clone(), *x)))?;
            ys.push(y);
        };

        // Pick return value based on samples.
        let cmp = |x: &f64, y: &f64| x.partial_cmp(y).unwrap();
        let mut ys = ys.into_iter();
        let y = match self {
            FuzzyRule::Unit(_, _) => ys.next(),
            FuzzyRule::And(_, _) => ys.min_by(cmp),
            FuzzyRule::Or(_, _) => ys.max_by(cmp),
        }.unwrap();
        Ok((out_category, out_term, y))
    }
}

pub struct FuzzerConfig {}
impl FuzzerConfig {
    pub fn new(
    ) -> Self {
        Self {}
    }
}

pub struct Fuzzer {
    categories: HashMap<String, FuzzySet>,
    outputs: HashMap<String, FuzzySet>,
    rules: Vec<FuzzyRule>,
    config: FuzzerConfig
}

impl Fuzzer {
    pub fn new(
    ) -> Self {
        Self {
            categories: HashMap::new(),
            outputs: HashMap::new(),
            rules: Vec::new(),
            config: FuzzerConfig::new()
        }
    }

    /// Applies rules to given to input and returns output fuzzy sets.
    pub fn apply(
        &self,
        values: &HashMap<String, f64>
    ) -> FuzzyResult<HashMap<String, FuzzySet>> {
        let mut results: HashMap<String, FuzzySetBuilder> = HashMap::new();
        for rule in self.rules.iter() {
            let (out_category, out_term, y) = rule.apply(&self, values)?;
            let base = self.outputs.get(&out_category)
                .ok_or(FuzzyError::InvalidCategory(out_category.clone()))?;
            let mut builder = results.remove(&out_category)
                .unwrap_or_else(|| FuzzySetBuilder::new(base));
            builder.value(&out_term, y);
            results.insert(out_category, builder);
        }
        println!("RES: {:?}", results);
        results.into_iter()
            .map(|(k, v)| v.build("out").map(|set| (k, set)))
            .collect()
    }

    pub fn rule(
        mut self,
        rule: FuzzyRule
    ) -> Self {
        self.rules.push(rule);
        self
    }

    pub fn fuzzify(
        mut self,
        ident: impl Into<String>,
        category: FuzzySet
    ) -> Self {
        self.categories.insert(ident.into(), category);
        self
    }

    pub fn defuzzify(
        mut self,
        ident: impl Into<String>,
        output: FuzzySet
    ) -> Self {
        self.outputs.insert(ident.into(), output);
        self
    }

    fn call(
        &self,
        point: &FuzzyValue
    ) -> FuzzyResult<f64> {
        let (category, term, x) = point;
        let set = self.categories.get(category)
            .ok_or(FuzzyError::InvalidCategory(category.clone()))?;
        let mut map = set.call(*x)?;
        map.remove(term)
            .ok_or(FuzzyError::InvalidTerm(term.clone()))
    }
}
