use std::collections::HashMap;
use std::fmt::Debug;
use std::{ops::Range, str::FromStr};

/// Subtract translations from vector a, that are fully implemented in vector b
pub fn trans_vec_sub_src(a: &Vec<Translation>, b: &Vec<Translation>) -> Vec<Translation> {
    let mut out = a.clone();
    for bt in b.iter() {
        let mut rest = vec![];
        for ot in out.iter() {
            let p = trans_sub_src(ot, bt);
            rest.extend(p);
        }
        out = rest;
    }
    out
}

/// Subtract translations from vector a, that are partially implemented in vector b based
/// on the destination range of a.
pub fn trans_vec_sub_dst(a: &Vec<Translation>, b: &Vec<Translation>) -> Vec<Translation> {
    let mut out = a.clone();
    for bt in b.iter() {
        let mut rest = vec![];
        for ot in out.iter() {
            let p = trans_sub_dst(ot, bt);
            rest.extend(p);
        }
        out = rest;
    }
    out
}

/// Shift translations from vector a, that are partially implemented in vector b based
/// on destination range of a
pub fn trans_vec_shift_overlaps(a: &Vec<Translation>, b: &Vec<Translation>) -> Vec<Translation> {
    let mut out = vec![];
    for bt in b.iter() {
        for at in a.iter() {
            out.extend(trans_shift_overlaps(at, bt));
        }
    }
    out
}

pub fn trans_shift_overlaps(a: &Translation, b: &Translation) -> Vec<Translation> {
    let mut out = vec![];
    let ao = a.out_range();

    // if b completely covers a we can snip_left and snip_right and return a
    //        |---a---|
    // |--------b-----------|
    if b.start() <= ao.start && b.end() >= ao.end {
        let mut t = b.clone();
        t.snip_left(ao.start - b.start());
        t.snip_right(b.end() - ao.end);
        // b should get the source range of a
        t.src = a.src;
        out.push(t);
        return out;
    }

    // if b is outside of a, we return nothing, since it's not overlapping
    if b.start() > ao.end || b.end() < ao.start {
        return out;
    }

    // if the new translation's source range is partially implemented in the input of an existing translation
    // we need to shrink the new translation's source range to the part that is not implemented
    // case 2 (left side)
    // |---a---|
    //    |---b---|
    if b.start() > ao.start && b.start() < ao.end && b.end() >= ao.end {
        // we need to shrink the new translation's source range to the part that is not implemented
        // and add the new translation to the output of that translation
        let mut t = b.clone();
        t.snip_right(b.end() - ao.end);
        // b should get the source range of a, but adds the shift
        // that is b.start - ao.start
        t.src = a.src + (b.start() - ao.start);
        out.push(t);
    }

    // case 3 (right side)
    //    |---a---|
    // |---b---|
    if b.start() <= ao.start && b.end() > ao.start && b.end() < ao.end {
        // we need to shrink the new translation's source range to the part that is not implemented
        // and add the new translation to the output of that translation
        let mut t = b.clone();
        t.snip_left(ao.start - b.start());
        // b should get the source range of a, but adds the shift
        t.src = a.src;
        out.push(t);
    }

    // case 4 (poke a hole in a)
    // |--------a-----------|
    //        |---b---|
    if b.start() > ao.start && b.end() < ao.end {
        // we don't need to snip b, we just need to
        // find the new source range
        let mut t = b.clone();
        // b should get the source range of a, but adds the shift
        t.src = a.src + (b.start() - ao.start);
        out.push(t);
    }

    out
}

pub fn trans_sub_dst(a: &Translation, b: &Translation) -> Vec<Translation> {
    let mut out = vec![];
    let ao = a.out_range();

    // if b completely covers a we can just return an empty vector
    //        |---a---|
    // |--------b-----------|
    if b.start() <= ao.start && b.end() >= ao.end {
        return out;
    }

    // if b is outside of a, we can just return a
    if b.start() > ao.end || b.end() <= ao.start {
        out.push(a.clone());
        return out;
    }

    // if the new translation's source range is partially implemented in the input of an existing translation
    // we need to shrink the new translation's source range to the part that is not implemented
    // case 2 (left side)
    // |---a---|
    //    |---b---|
    if b.start() > ao.start && b.start() < ao.end && b.end() >= ao.end {
        // we need to shrink the new translation's source range to the part that is not implemented
        // and add the new translation to the output of that translation
        let mut t = a.clone();
        t.snip_right(ao.end - b.start());
        out.push(t);
    }

    // case 3 (right side)
    //    |---a---|
    // |---b---|
    if b.start() <= ao.start && b.end() > ao.start && b.end() < ao.end {
        // we need to shrink the new translation's source range to the part that is not implemented
        // and add the new translation to the output of that translation
        let mut t = a.clone();
        t.snip_left(b.end() - ao.start);
        out.push(t);
    }

    // case 4 (poke a hole in a)
    // |--------a-----------|
    //        |---b---|
    if b.start() > ao.start && b.end() < ao.end {
        // we need to split the new translation in two parts,
        // or snip left, and add another one that snips right.

        let mut t = a.clone();
        t.snip_right(ao.end - b.start());
        out.push(t);

        let mut t = a.clone();
        t.snip_left(b.end() - ao.start);
        out.push(t);
    }
    out
}

/// Subtract translation b from translation a looking at inputs, returning remaining parts.
///
/// # Examples
///
/// ```
/// let a = Translation { src: 10, dst: 50, rng: 10 };
/// let b = Translation { src: 15, dst: 70, rng: 5 };
/// let c = trans_sub_src(&a, &b);
/// assert_eq!(c, vec![Translation { src: 10, dst: 50, rng: 5 }]);
/// ```
///
pub fn trans_sub_src(a: &Translation, b: &Translation) -> Vec<Translation> {
    let mut out = vec![];
    // if b completely covers a we can just return an empty vector
    //        |---a---|
    // |--------b-----------|
    if b.start() <= a.start() && b.end() >= a.end() {
        return out;
    }

    // if b is outside of a, we can just return a
    if b.start() > a.end() || b.end() < a.start() {
        out.push(a.clone());
        return out;
    }

    // if the new translation's source range is partially implemented in the input of an existing translation
    // we need to shrink the new translation's source range to the part that is not implemented
    // case 2 (left side)
    // |---a---|
    //    |---b---|
    if b.start() > a.start() && b.start() < a.end() && b.end() >= a.end() {
        // we need to shrink the new translation's source range to the part that is not implemented
        // and add the new translation to the output of that translation
        let mut t = a.clone();
        t.snip_right(a.end() - b.start());
        out.push(t);
    }

    // case 3 (right side)
    //    |---a---|
    // |---b---|
    if b.start() <= a.start() && b.end() > a.start() && b.end() < a.end() {
        // we need to shrink the new translation's source range to the part that is not implemented
        // and add the new translation to the output of that translation
        let mut t = a.clone();
        t.snip_left(b.end() - a.start());
        out.push(t);
    }

    // case 4 (poke a hole in a)
    // |--------a-----------|
    //        |---b---|
    if b.start() > a.start() && b.end() < a.end() {
        // we need to split the new translation in two parts,
        // or snip left, and add another one that snips right.

        let mut t = a.clone();
        t.snip_right(a.end() - b.start());
        out.push(t);

        let mut t = a.clone();
        t.snip_left(b.end() - a.start());
        out.push(t);
    }
    out
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Translation {
    src: u64,
    dst: u64,
    rng: u64,
}

impl Translation {
    fn in_range(&self, value: u64) -> bool {
        value >= self.src && value < self.src + self.rng
    }

    fn translate(&self, value: u64) -> u64 {
        if self.in_range(value) {
            self.dst + (value - self.src)
        } else {
            value
        }
    }

    // start is inclusive, since we are using ranges
    pub fn start(&self) -> u64 {
        self.src
    }

    // end is exclusive, since we are using ranges
    pub fn end(&self) -> u64 {
        self.src + self.rng
    }

    pub fn range(&self) -> Range<u64> {
        self.start()..self.end()
    }

    pub fn out_range(&self) -> Range<u64> {
        self.dst..self.dst + self.rng
    }

    pub fn snip_left(&mut self, amount: u64) {
        if amount > self.rng {
            panic!("snip_left: amount is larger than rng");
        }

        self.src += amount;
        self.rng -= amount;
        self.dst += amount;
    }

    pub fn snip_right(&mut self, amount: u64) {
        if amount > self.rng {
            panic!("snip_right: amount is larger than rng");
        }

        self.rng -= amount;
    }

    // Translate a range, returning a tuple containing the translated ranges
    // and the ranges that were not translated
    pub fn translate_range(&self, r: &Range<u64>) -> (Vec<Range<u64>>, Vec<Range<u64>>) {
        let mut translated = vec![];
        let mut not_translated = vec![];
        // Several cases:

        // 2a. the translation is fully inside the range
        // |------------------range---------------|
        //    |---translation---|
        if self.start() >= r.start && self.end() <= r.end {
            // In this case, we need to split the range in three parts
            // the first part is from the start of the range to the start of the translation
            if r.start < self.start() {
                not_translated.push(r.start..self.start());
            }
            // the second part is the translation
            translated.push(self.translate(self.start())..(self.translate(self.end() - 1) + 1));
            // the third part is from the end of the translation to the end of the range
            if r.end > self.end() {
                not_translated.push(self.end()..r.end);
            }
            return (translated, not_translated);
        }

        // 2b. the range is fully inside the translation
        //    |---range---|
        // |------------------translation---------------|
        if r.start >= self.start() && r.end <= self.end() {
            translated.push(self.translate(r.start)..self.translate(r.end));
            return (translated, not_translated);
        }

        // 3a. the translation is partially inside the range
        // But the range starts before the trans
        // and ends inside the translation
        // |------range-----|
        //    |---translation---|
        if r.start < self.start() && r.end > self.start() && r.end < self.end() {
            // in this case, we need to split the range in two parts
            // the first part is from the start of the range to the start of the translation
            not_translated.push(r.start..self.start());
            // the second part is the translation start to the end of the range, but translated
            translated.push(self.translate(self.start())..(self.translate(r.end - 1) + 1));
            return (translated, not_translated);
        }

        // 3b. the translation is partially inside the range
        // but the range starts inside the translation
        // and ends after the translation
        //    |------range-----|
        // |---translation---|
        if r.start >= self.start() && r.start <= self.end() && r.end > self.end() {
            // in this case, we need to split the range in two parts
            // the first part is from the start of the range to the end of the translation
            translated.push(self.translate(r.start)..self.translate(self.end()));
            // the second part is the translation start to the end of the range, not translated
            not_translated.push(self.end()..r.end);
            return (translated, not_translated);
        }

        // 1. the translation is fully outside the range
        // This is the falback case
        // |---translation---|
        //                         |---range---|
        // or
        //                         |---translation---|
        // |---range---|
        // In this case, we just add the range to the output
        if self.end() <= r.start || self.start() >= r.end {
            not_translated.push(r.clone());
            return (translated, not_translated);
        }

        // we should not get here
        panic!("translate_range: unhandled case");
    }
}

impl FromStr for Translation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        let dst = iter.next().unwrap().parse().unwrap();
        let src = iter.next().unwrap().parse().unwrap();
        let rng = iter.next().unwrap().parse().unwrap();

        Ok(Translation { src, dst, rng })
    }
}

#[derive(Debug, PartialEq)]
pub struct Map {
    translations: Vec<Translation>,
}

impl Map {
    fn translate(&self, value: u64) -> u64 {
        for t in self.translations.iter() {
            if t.in_range(value) {
                return t.translate(value);
            }
        }
        value
    }

    fn range_map(&self) -> HashMap<Range<u64>, Range<u64>> {
        let mut output = HashMap::new();

        for t in self.translations.iter() {
            let out_range = t.translate(t.start())..t.translate(t.end() - 1) + 1;
            output.insert(t.range(), out_range);
        }

        output
    }

    fn detect_overlaps(&self) {
        // get the range map for this map
        let rangemap = self.range_map();
        // check if the rangemap contains any overlapping ranges as keys of
        // the hashmap
        // if so, panic

        // get a vector with the keys of the hashmap
        let keys = rangemap.keys().collect::<Vec<_>>();

        for (i, k) in keys.iter().enumerate() {
            for (j, k2) in keys.iter().enumerate() {
                // skip if same one
                if i == j {
                    continue;
                }
                // if the ranges are overlapping
                if k.start < k2.end && k.end > k2.start {
                    panic!("overlap detected: {:?} and {:?}", k, k2);
                }
            }
        }
    }

    /// Add another map to this map.
    /// The naive approach would be to just add the translations from the other map
    /// to this map. However, this would not work in all cases.
    /// We need to revise the the translations in this map to make sure that they
    /// accommodate the translations in the other map.
    fn add_map(&mut self, other: &mut Map) {
        self.add_adapt_translations(&mut other.translations);
    }

    /// Add a translation to this map, adapting the existing translations
    /// to accommodate the new translation
    pub fn add_adapt_translations(&mut self, ts: &mut Vec<Translation>) {
        // We are in dire need of a better algorithm here.
        // Adapting an existing map to an incoming map should follow
        // an approach where several steps are taken in a specific order,
        // to ensure that there's never any overlap between the translations.

        // the first stap is to plan ahead and create a list of any "new"
        // translations that should be added after the existing translations
        // have been adapted. These are the translations that have inputs that
        // are not fully implemented in the existing translations.
        // In other words, we will create a list of these translations, and
        // "carve out" the parts that are already implemented in the existing
        // translations. We will add these ones only at the end. It will be safe
        // to do so, since we know that the existing translations will not
        // handle these inputs.
        let mut new_inputs = ts.clone();
        new_inputs = trans_vec_sub_src(&new_inputs, &self.translations);

        // The second step is to carve holes in the existing translations that
        // have outputs that are partially implemented in the new translations.
        // We will not fill up these holes yet, since they might influence
        // eachother.
        let existing_with_holes = trans_vec_sub_dst(&self.translations, &ts);

        // The third step is to fill up the holes that were created in the
        // second step. This will be done by adding new translations that
        // will fill up the holes. These new translations will be added to the
        // as a last step, to ensure that they do not influence the other
        // translations. The special thing about these translations, is that
        // they will take over the source of the existing translation, and
        // translate it to the destination of the new translation.
        let new_shifted = trans_vec_shift_overlaps(&self.translations, &ts);

        // the new set of translations is the sum of new_inputs, existing_with_holes, and new_shifted
        self.translations = vec![];
        self.translations.extend(new_inputs);
        self.translations.extend(existing_with_holes);
        self.translations.extend(new_shifted);
        return;
    }

    fn translate_range(&self, r: &Range<u64>) -> Vec<Range<u64>> {
        let mut output = vec![];
        // keep track of the remaining ranges
        let mut remaining = vec![r.clone()];

        for t in self.translations.iter() {
            let mut to_translate = vec![];
            for r in remaining.iter() {
                let (mut translated, mut not_translated) = t.translate_range(r);
                output.append(&mut translated);
                to_translate.append(&mut not_translated);
            }
            remaining = to_translate;
        }

        // if there are still ranges to translate, we add them to the output
        output.append(&mut remaining);

        // if output is empty, we just return the input range
        if output.is_empty() {
            output.push(r.clone());
        }

        Map::simplify_ranges(output)
    }

    /// This function takes a list of ranges and simplifies them
    /// by merging overlapping ranges
    /// and removing ranges that are fully contained in other ranges
    pub fn simplify_ranges(ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        // if there is only one range, we can return it directly
        if ranges.len() == 1 {
            return ranges;
        }

        // we sort the ranges by their start
        let mut ranges = ranges;
        ranges.sort_by(|a, b| a.start.cmp(&b.start));

        // we iterate over the ranges
        let mut output = vec![];

        let mut current = ranges[0].clone();

        for r in ranges.iter().skip(1) {
            // if the current range is fully contained in the next range
            // we can skip it
            if current.start >= r.start && current.end <= r.end {
                continue;
            }

            // if the current range overlaps with the next range
            // we merge them
            if current.end >= r.start {
                current = current.start..r.end;
                continue;
            }

            // if the current range is fully outside the next range
            // we can add it to the output
            if current.end < r.start {
                output.push(current.clone());
                current = r.clone();
                continue;
            }
        }

        // we need to add the last range
        output.push(current);

        output
    }

    pub fn lowest_in_ranges(&self, ranges: Vec<Range<u64>>) -> u64 {
        ranges
            .iter()
            .map(|r| self.translate_range(r))
            .flatten()
            .map(|r| r.start)
            .min()
            .unwrap()
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let translations = s
            .lines()
            .filter(|l| !l.contains("map"))
            .map(|l| l.parse().unwrap())
            .collect();

        Ok(Map { translations })
    }
}

pub fn parse_input(input: &str) -> (Vec<u64>, Vec<Map>) {
    let mut iter = input.split("\n\n");

    // take the first line
    let seeds = iter
        .next()
        .unwrap()
        .replace("seeds: ", "")
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let categories = iter.map(|s| s.parse().unwrap()).collect();

    (seeds, categories)
}

pub fn solve(input: &str) -> u64 {
    let (seeds, categories) = parse_input(input);

    seeds
        .iter()
        .map(|s| categories.iter().fold(*s, |acc, c| c.translate(acc)))
        .min()
        .unwrap()
}

pub fn solve2(input: &str) -> u64 {
    let (seeds, categories) = parse_input(input);

    // transform the seeds into ranges
    let mut ranges = seeds
        .chunks(2)
        .map(|c| c[0]..c[0] + c[1])
        .collect::<Vec<_>>();

    for c in categories {
        let mut new_ranges = vec![];
        for r in ranges.iter() {
            let out_r = c.translate_range(r);
            new_ranges.extend(out_r);
        }
        ranges = new_ranges;
    }

    // return the lowest value in all of the ranges
    ranges.iter().map(|r| r.start).min().unwrap()
}

pub fn solve2b(input: &str) -> u64 {
    let (seeds, mut categories) = parse_input(input);

    // transform the seeds into ranges
    // take the array of values and split it into pairs
    let ranges = seeds
        .chunks(2)
        .map(|c| c[0]..c[0] + c[1])
        .collect::<Vec<_>>();

    let mut basemap: Map = Map {
        translations: vec![],
    };

    for c in categories.iter_mut() {
        basemap.add_map(c);
        basemap.detect_overlaps();
    }

    basemap.lowest_in_ranges(ranges)
}

pub fn main() {
    let input = include_str!("../../input/day05.txt");

    let output = solve(input);

    println!("Part 1: {}", output);

    let output = solve2b(input);

    println!("Part 2: {}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_vec_eq<T: PartialEq + Debug>(a: Vec<T>, b: Vec<T>) {
        assert_eq!(b.iter().all(|x| a.contains(x)), true, "{:?} != {:?}", a, b);
    }

    #[test]
    fn test_solve() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

        let expected = 35;

        let output = solve(input);

        assert_eq!(output, expected);
    }

    #[test]
    fn test_solve2() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

        let expected = 46;

        let output = solve2b(input);

        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_maps() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15"#;

        let (seeds, _) = parse_input(input);

        let expected_seeds = vec![79, 14, 55, 13];

        assert_eq!(seeds, expected_seeds);
    }

    #[test]
    fn test_map_parsing() {
        let input = r#"50 98 2"#;

        let expected = Map {
            translations: vec![Translation {
                src: 98,
                dst: 50,
                rng: 2,
            }],
        };

        let output = input.parse::<Map>().unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_map_translation() {
        let input = r#"50 98 2"#;

        let map = input.parse::<Map>().unwrap();

        assert_eq!(map.translate(97), 97);
        assert_eq!(map.translate(98), 50);
        assert_eq!(map.translate(99), 51);
        assert_eq!(map.translate(100), 100);
    }

    #[test]
    fn test_range_translation() {
        let input = r#"50 98 2"#;

        let map = input.parse::<Map>().unwrap();

        assert_vec_eq(map.translate_range(&(95..97)), vec![(95..97)]);
        assert_vec_eq(map.translate_range(&(95..99)), vec![(95..98), (50..51)]);
        assert_vec_eq(map.translate_range(&(95..100)), vec![(95..98), (50..52)]);
        assert_vec_eq(
            map.translate_range(&(95..101)),
            vec![(95..98), (50..52), (100..101)],
        );
    }

    #[test]
    fn test_range_translation2() {
        let input = r#"52 50 48"#;
        let map = input.parse::<Map>().unwrap();

        assert_vec_eq(map.translate_range(&(79..93)), vec![(81..95)]);
    }

    #[test]
    fn test_range_translation3() {
        let input = r#"100 5 5
200 10 5"#;
        let map = input.parse::<Map>().unwrap();

        assert_vec_eq(map.translate_range(&(0..10)), vec![(0..5), (100..105)]);

        assert_vec_eq(
            map.translate_range(&(0..15)),
            vec![(0..5), (100..105), (200..205)],
        );

        assert_vec_eq(
            map.translate_range(&(0..20)),
            vec![(0..5), (100..105), (200..205), (15..20)],
        );
    }

    #[test]
    fn test_map_normalization() {
        let input = r#"100 5 5
200 10 5"#;
        let mut map = input.parse::<Map>().unwrap();

        // The next approach is going to be to come up with some kind of
        // map normalization or flattening.
        // It would involve creating a matrix that can be used to represent
        // the operations that the translations would do.
        // If we're able to present all translations in the map as a matrix,
        // we could probably much more easily figure out the lowest value in
        // the matrix.
        // For instance, the above map would be represented as:
        // 5..10 => 100..105
        // 10..15 => 200..205
        let range_map = map.range_map();

        // assert that the rangemap contains the expected values
        assert_eq!(range_map.get(&(5..10)), Some(&(100..105)));
        assert_eq!(range_map.get(&(10..15)), Some(&(200..205)));

        //
        // If we add another map that has the following translations:
        let mut map2 = "0 102 2".parse::<Map>().unwrap();
        map.add_map(&mut map2);

        // It would change our representation to:
        // 5..7 => 100..102
        // 7..9 => 0..2
        // 9..10 => 104..105
        // 10..15 => 200..205
        // 102..104 => 0..2
        let range_map = map.range_map();

        // assert that the rangemap contains the expected values
        assert_eq!(range_map.get(&(5..7)), Some(&(100..102)));
        assert_eq!(range_map.get(&(7..9)), Some(&(0..2)));
        assert_eq!(range_map.get(&(9..10)), Some(&(104..105)));
        assert_eq!(range_map.get(&(10..15)), Some(&(200..205)));
        assert_eq!(range_map.get(&(102..104)), Some(&(0..2)));

        //
        // If we add another map that has the following translations:
        let mut map3 = "30 203 5".parse::<Map>().unwrap();
        map.add_map(&mut map3);

        // It would change our representation to:
        // 5..7 => 100..102
        // 7..9 => 0..2
        // 9..10 => 104..105
        // 10..13 => 200..203
        // 13..15 => 30..32
        // 102..104 => 0..2
        // 203..208 => 30..35
        let range_map = map.range_map();

        // assert that the rangemap contains the expected values
        assert_eq!(range_map.get(&(5..7)), Some(&(100..102)));
        assert_eq!(range_map.get(&(7..9)), Some(&(0..2)));
        assert_eq!(range_map.get(&(9..10)), Some(&(104..105)));
        assert_eq!(range_map.get(&(10..13)), Some(&(200..203)));
        assert_eq!(range_map.get(&(13..15)), Some(&(30..32)));
        assert_eq!(range_map.get(&(102..104)), Some(&(0..2)));
        assert_eq!(range_map.get(&(203..208)), Some(&(30..35)));

        // If we manage to implement this, than for each range of seeds we can
        // look up the input items that are in that range, and then look at the
        // lowest value in the matrix.

        // For instance, for an input seed range of 4..10, the lowest value in
        // output would be 0 (for seed 7).
        assert_eq!(
            map.lowest_in_ranges(vec![4..10]),
            0,
            "lowest_in_ranges(4..10)",
        );

        // For an input range of 14..15, the lowest value in output would be 31
        assert_eq!(
            map.lowest_in_ranges(vec![14..15]),
            31,
            "lowest_in_ranges(14..15)",
        );

        // for an input range of 80..120, the lowest value in output would be 0
        assert_eq!(
            map.lowest_in_ranges(vec![80..120]),
            0,
            "lowest_in_ranges(80..120)",
        );

        // for an input range of 200..205, the lowest value in output would be 30
        assert_eq!(
            map.lowest_in_ranges(vec![200..205]),
            30,
            "lowest_in_ranges(200..205)",
        );

        // In this case, the lowest value is 0, so we can just return that.
        // Looking quite critically at this, it seems that we can just
        // look at the last map,
    }

    #[test]
    fn test_map_normalization_entirely_within() {
        let mut map = "100 0 50".parse::<Map>().unwrap();

        let mut map2 = "210 110 30".parse::<Map>().unwrap();

        map.add_map(&mut map2);

        let range_map = map.range_map();

        // assert that the rangemap contains the expected values
        assert_eq!(range_map.get(&(0..10)), Some(&(100..110)));
        assert_eq!(range_map.get(&(10..40)), Some(&(210..240)));
        assert_eq!(range_map.get(&(40..50)), Some(&(140..150)));
    }

    #[test]
    fn test_map_normalization_right_side() {
        let mut map = "39 15 15".parse::<Map>().unwrap();

        let mut map2 = "4 15 37".parse::<Map>().unwrap();

        map.add_map(&mut map2);

        //       15       30
        //       39       54
        //       |--------|
        //  15       52
        //  4        41
        //  |--------|
        let range_map = map.range_map();

        // assert that the rangemap contains the expected values
        assert_eq!(range_map.get(&(28..30)), Some(&(52..54)));
        assert_eq!(range_map.get(&(15..28)), Some(&(28..41)));
    }

    #[test]
    fn test_map_normalization_right_side2() {
        let mut map = "39 0 14".parse::<Map>().unwrap();

        let mut map2 = "3 14 38".parse::<Map>().unwrap();

        map.add_map(&mut map2);

        //       0        14
        //       39       53
        //       |--------|
        //  14       52
        //  3        41
        //  |--------|
        let range_map = map.range_map();

        // assert that the rangemap contains the expected values
        assert_eq!(range_map.get(&(13..14)), Some(&(52..53)));
        assert_eq!(range_map.get(&(0..13)), Some(&(28..41)));
    }

    #[test]
    fn test_overlap_check() {
        let mut map = "50 98 2
52 50 48"
            .parse::<Map>()
            .unwrap();

        // the map should have only 2 translations!
        assert_eq!(map.translations.len(), 2);

        let mut map2 = "0 15 37
        37 52 2
        39 0 15"
            .parse::<Map>()
            .unwrap();

        map.add_map(&mut map2);

        map.detect_overlaps();
    }

    #[test]
    fn test_overlap_check2() {
        let mut map = "52 50 48".parse::<Map>().unwrap();

        let mut map2 = "37 52 2".parse::<Map>().unwrap();

        // 50       98
        // 52       100
        // |--------|
        // 52   54
        // 37   39
        // |----|

        // resulting map should be
        // 50..52 => 37..39
        // 52..98 => 54..100

        map.add_map(&mut map2);

        map.detect_overlaps();
    }

    #[test]
    fn test_overlap_check3() {
        let mut map = "50 98 2
 52 50 48
 0 15 35
 39 0 15"
            .parse::<Map>()
            .unwrap();

        let mut map2 = "0 11 42".parse::<Map>().unwrap();

        // 0    15 15      50 50       98 98       100
        // 39   54 0       35 52      100 50       52
        // |----|  |-------|  |-------|   |--------|
        // 11..53 => 0..42
        //

        map.add_map(&mut map2);

        map.detect_overlaps();
    }

    #[test]
    fn test_overlap_check4() {
        let mut map = "0 15 35".parse::<Map>().unwrap();

        let mut map2 = "0 11 42".parse::<Map>().unwrap();

        map.add_map(&mut map2);

        //     15      50
        //     0       35
        //     |-------|
        // 11..53 => 0..42
        // 11  15 15  26 26   50 50    53
        // 0   4  0   11 0    24 39    42
        // |---|  |---|  |----|  |-----|

        map.detect_overlaps();
    }

    #[test]
    fn test_overlap_check5() {
        let mut map = "0 15 35
39 0 15"
            .parse::<Map>()
            .unwrap();

        let mut map2 = "0 11 42".parse::<Map>().unwrap();

        map.add_map(&mut map2);

        //  0        15
        //  39       54
        //  |--------|
        //     50  53

        map.detect_overlaps();
    }

    #[test]
    fn test_overlap_check6() {
        let mut map = "52 50 48".parse::<Map>().unwrap();

        let mut map2 = "49 53 8
0 11 42"
            .parse::<Map>()
            .unwrap();

        map.add_map(&mut map2);

        //  50           98
        //  52           100
        //  |------------|
        //    53 61
        //    49 57
        //    |---|
        map.detect_overlaps();
    }

    #[test]
    fn test_add_map_to_empty_map() {
        let mut map = Map {
            translations: vec![],
        };

        let mut map2 = "50 98 2
52 50 48"
            .parse::<Map>()
            .unwrap();

        map.add_map(&mut map2);

        // the map should have only 2 translations!
        assert_eq!(map.translations.len(), 2);
    }

    #[test]
    fn test_translation_impl() {
        let t = Translation {
            src: 98,
            dst: 50,
            rng: 2,
        };

        assert_eq!(t.in_range(97), false);
        assert_eq!(t.in_range(98), true);
        assert_eq!(t.in_range(99), true);
        assert_eq!(t.in_range(100), false);

        assert_eq!(t.start(), 98);
        assert_eq!(t.end(), 100);
    }

    #[test]
    fn test_simplify_ranges() {
        let input = vec![(0..10), (5..15), (20..30), (25..35)];

        let expected = vec![(0..15), (20..35)];

        let output = Map::simplify_ranges(input);

        assert_vec_eq(output, expected);
    }

    #[test]
    fn test_trans_sub_src_cover() {
        let a = Translation {
            src: 10,
            dst: 50,
            rng: 10,
        };
        let b = Translation {
            src: 5,
            dst: 70,
            rng: 20,
        };
        let c = trans_sub_src(&a, &b);
        assert_eq!(c, vec![]);
    }

    #[test]
    fn test_trans_sub_src_out1() {
        let a = Translation {
            src: 10,
            dst: 50,
            rng: 10,
        };
        let b = Translation {
            src: 2,
            dst: 70,
            rng: 4,
        };
        let c = trans_sub_src(&a, &b);
        assert_eq!(c, vec![a]);
    }

    #[test]
    fn test_trans_sub_src_out2() {
        let a = Translation {
            src: 10,
            dst: 50,
            rng: 10,
        };
        let b = Translation {
            src: 23,
            dst: 70,
            rng: 4,
        };
        let c = trans_sub_src(&a, &b);
        assert_eq!(c, vec![a]);
    }

    #[test]
    fn test_trans_sub_src_right_snip() {
        let a = Translation {
            src: 10,
            dst: 50,
            rng: 10,
        };
        let b = Translation {
            src: 15,
            dst: 70,
            rng: 5,
        };
        let c = trans_sub_src(&a, &b);
        assert_eq!(
            c,
            vec![Translation {
                src: 10,
                dst: 50,
                rng: 5
            }]
        );
    }

    #[test]
    fn test_trans_sub_src_left_snip() {
        let a = Translation {
            src: 10,
            dst: 50,
            rng: 10,
        };
        let b = Translation {
            src: 5,
            dst: 70,
            rng: 10,
        };
        let c = trans_sub_src(&a, &b);
        assert_eq!(
            c,
            vec![Translation {
                src: 15,
                dst: 55,
                rng: 5
            }]
        );
    }

    #[test]
    fn test_trans_sub_src_poke_hole() {
        let a = Translation {
            src: 10,
            dst: 50,
            rng: 10,
        };
        let b = Translation {
            src: 12,
            dst: 70,
            rng: 6,
        };
        let c = trans_sub_src(&a, &b);
        assert_eq!(
            c,
            vec![
                Translation {
                    src: 10,
                    dst: 50,
                    rng: 2
                },
                Translation {
                    src: 18,
                    dst: 58,
                    rng: 2
                }
            ]
        );
    }

    #[test]
    fn test_trans_sub_dst_example() {
        let a = Translation {
            src: 50,
            dst: 52,
            rng: 48,
        };
        let b = Translation {
            src: 15,
            dst: 0,
            rng: 37,
        };
        let c = trans_sub_dst(&a, &b);
        assert_eq!(
            c,
            vec![Translation {
                src: 50,
                dst: 52,
                rng: 48
            }]
        );
    }

    #[test]
    fn test_trans_sub_dst_poke_hole() {
        let a = Translation {
            src: 10,
            dst: 50,
            rng: 10,
        };
        let b = Translation {
            src: 52,
            dst: 70,
            rng: 6,
        };
        let c = trans_sub_dst(&a, &b);
        assert_eq!(
            c,
            vec![
                Translation {
                    src: 10,
                    dst: 50,
                    rng: 2
                },
                Translation {
                    src: 18,
                    dst: 58,
                    rng: 2
                }
            ]
        );
    }

    #[test]
    fn test_trans_shift_overlap_cover() {
        let a = Translation {
            src: 10,
            dst: 50,
            rng: 10,
        };
        let b = Translation {
            src: 40,
            dst: 70,
            rng: 30,
        };
        let c = trans_shift_overlaps(&a, &b);
        assert_eq!(
            c,
            vec![Translation {
                src: 10,
                dst: 80,
                rng: 10
            }]
        );
    }

    #[test]
    fn test_trans_shift_overlap_outside() {
        let a = Translation {
            src: 10,
            dst: 50,
            rng: 10,
        };
        let b = Translation {
            src: 40,
            dst: 70,
            rng: 5,
        };
        let c = trans_shift_overlaps(&a, &b);
        assert_eq!(c, vec![]);
    }

    #[test]
    fn test_trans_shift_overlap_left() {
        let a = Translation {
            src: 10,
            dst: 50,
            rng: 10,
        };
        let b = Translation {
            src: 55,
            dst: 70,
            rng: 10,
        };
        let c = trans_shift_overlaps(&a, &b);
        assert_eq!(
            c,
            vec![Translation {
                src: 15,
                dst: 70,
                rng: 5
            }]
        );
    }

    #[test]
    fn test_trans_shift_overlap_right() {
        let a = Translation {
            src: 10,
            dst: 50,
            rng: 10,
        };
        let b = Translation {
            src: 45,
            dst: 70,
            rng: 10,
        };
        let c = trans_shift_overlaps(&a, &b);
        assert_eq!(
            c,
            vec![Translation {
                src: 10,
                dst: 75,
                rng: 5
            }]
        );
    }

    #[test]
    fn test_trans_shift_overlap_poke() {
        let a = Translation {
            src: 10,
            dst: 50,
            rng: 30,
        };
        let b = Translation {
            src: 60,
            dst: 100,
            rng: 10,
        };
        let c = trans_shift_overlaps(&a, &b);
        assert_eq!(
            c,
            vec![Translation {
                src: 20,
                dst: 100,
                rng: 10
            }]
        );
    }
}
