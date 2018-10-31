//! Bible reference parser
//!
//! Created by Grigory Avdyushin <avdyushin.g@gmail.com> on 17/10/2018.
//! Copyright © 2018 Grigory Avdyushin. All rights reserved.
//!
//! #Examples
//!
//! ```rust
//! let refs = bible_reference_rs::parse("Gen 1:1-3, Act 9");
//!
//! assert_eq!(refs.len(), 2);
//! assert_eq!(refs[0].book, "Gen");
//! assert_eq!(refs[0].locations[0].chapters, [1]);
//! assert_eq!(refs[0].locations[0].verses, Some(vec![1, 2, 3]));
//! ```

#![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

/// Verse location representation
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct VerseLocation {
    /// Chapters
    pub chapters: Vec<u8>,
    /// Verses
    pub verses: Option<Vec<u8>>,
}

/// Verse reference representation
#[derive(Debug)]
pub struct BibleReference {
    /// Book name
    pub book: String,
    /// Verse locations
    pub locations: Vec<VerseLocation>,
}

// Single chapter: 1
// Range: 1-2
// Sequence: 1,4
// Mixed chapters 1-2,4
// Single verse: 1:1
// Range: 1:1-3
// Sequence: 1:1,3
// Mixed verses: 1:1-2,4
static VERSES_LOCATION_PATTERN: &'static str = "(?P<Chapter>1?[0-9]?[0-9])\
                                                (-(?P<ChapterEnd>\\d+)|,\\s*(?P<ChapterNext>\\d+))*\
                                                (:\\s*(?P<Verse>\\d+))?\
                                                (-(?P<VerseEnd>\\d+)|,\\s*(?P<VerseNext>\\d+))*";

// Gen 1:1, 2
// 3 King 1:3-4
// II Ki. 3:12-14, 25
static BIBLE_REFERENCE_PATTERN: &'static str = "(?P<Book>(([1234]|I{1,4})\\s*)?\\pL+\\.?)\\s*\
                                                (?P<Locations>(\
                                                (?P<Chapter>1?[0-9]?[0-9])\
                                                (-(?P<ChapterEnd>\\d+)|,\\s*(?P<ChapterNext>\\d+))*\
                                                (:\\s*(?P<Verse>\\d+))?\
                                                (-(?P<VerseEnd>\\d+)|,\\s*(?P<VerseNext>\\d+))*\
                                                \\s?)+)";

/// Parses string into references
pub fn parse(string: &str) -> Vec<BibleReference> {
    lazy_static! {
        static ref RE: Regex = Regex::new(BIBLE_REFERENCE_PATTERN).unwrap();
    }

    RE.captures_iter(string)
        .flat_map(|matches| {
            if let (Some(book), Some(locations)) = (matches.name("Book"), matches.name("Locations"))
            {
                Some(BibleReference {
                    book: book.as_str().to_string(),
                    locations: parse_locations(locations.as_str()),
                })
            } else {
                None
            }
        }).collect()
}

/// Parses string into locations
fn parse_locations(string: &str) -> Vec<VerseLocation> {
    lazy_static! {
        static ref RE: Regex = Regex::new(VERSES_LOCATION_PATTERN).unwrap();
    }

    RE.captures_iter(string)
        .flat_map(|matches| {
            let chapter = match matches.name("Chapter") {
                Some(group) => match group.as_str().parse().ok() {
                    Some(chapter) => chapter,
                    None => return None,
                },
                None => return None,
            };

            let chapter_end = match matches.name("ChapterEnd") {
                Some(group) => group.as_str().parse().ok(),
                None => None,
            };
            let chapter_next = match matches.name("ChapterNext") {
                Some(group) => group.as_str().parse().ok(),
                None => None,
            };

            let chapters_range = match (chapter, chapter_next, chapter_end) {
                (ch, None, None) => vec![ch],
                (ch, Some(next), None) => vec![ch, next],
                (ch, None, Some(end)) => (ch..=end).collect(),
                (ch, Some(next), Some(end)) => {
                    let mut vec: Vec<u8> = (ch..=end).collect();
                    vec.push(next);
                    vec
                }
            };

            let verse = match matches.name("Verse") {
                Some(group) => group.as_str().parse().ok(),
                None => None,
            };
            let verse_next = match matches.name("VerseNext") {
                Some(group) => group.as_str().parse().ok(),
                None => None,
            };
            let verse_end = match matches.name("VerseEnd") {
                Some(group) => group.as_str().parse().ok(),
                None => None,
            };

            let verses_range = match (verse, verse_next, verse_end) {
                (Some(verse), None, None) => Some(vec![verse]),
                (Some(verse), Some(next), None) => Some(vec![verse, next]),
                (Some(verse), None, Some(end)) => Some((verse..=end).collect()),
                (Some(verse), Some(next), Some(end)) => {
                    let mut vec: Vec<u8> = (verse..=end).collect();
                    vec.push(next);
                    Some(vec)
                }
                _ => None,
            };

            Some(VerseLocation {
                chapters: chapters_range,
                verses: verses_range,
            })
        }).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_verses_location() {
        let v = VerseLocation {
            chapters: vec![1],
            verses: Some(vec![1, 2]),
        };
        assert_eq!(v.chapters, vec![1]);
        assert_eq!(v.verses, Some(vec![1, 2]));
    }

    #[test]
    fn test_chapter_location() {
        let v = VerseLocation {
            chapters: vec![1, 3],
            verses: None,
        };
        assert_eq!(v.chapters, vec![1, 3]);
        assert_eq!(v.verses, None);
    }

    #[test]
    fn test_bible_reference() {
        let v = VerseLocation {
            chapters: vec![1],
            verses: Some(vec![1, 2]),
        };
        let r = BibleReference {
            book: String::from("Gen"),
            locations: vec![v],
        };
        assert_eq!(r.book, "Gen");
        assert_eq!(r.locations[0].chapters, [1]);
        assert_eq!(r.locations[0].verses, Some(vec![1, 2]));
    }

    #[test]
    fn test_parse_simple() {
        let refs = parse("1Cor 1:1");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].book, "1Cor");
        assert_eq!(refs[0].locations[0].chapters, [1]);
        assert_eq!(refs[0].locations[0].verses, Some(vec![1]));
    }

    #[test]
    fn test_parse_singleline() {
        let refs = parse("II Ki. 3:12-14, 25");

        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].book, "II Ki.");
        assert_eq!(refs[0].locations[0].chapters, [3]);
        assert_eq!(refs[0].locations[0].verses, Some(vec![12, 13, 14, 25]));
    }

    #[test]
    fn test_wrong_input_1() {
        let refs = parse("123");
        assert_eq!(refs.len(), 0);
    }

    #[test]
    fn test_wrong_input_2() {
        let refs = parse("1 234 3:4");
        assert_eq!(refs.len(), 0);
    }

    #[test]
    fn test_parse_multiline() {
        let refs = parse(
            "Daily readings are Быт 1;\
             Исх 1:2,4;\
             1 Пет 5-8, 10.\
             Also take a look in:
             Rev 2,4;\
             Jh 1:2-4,7\
             Gen 1:1-2 2:2,5",
        );

        assert_eq!(refs.len(), 6);
        assert_eq!(refs[0].book, "Быт");
        assert_eq!(refs[0].locations[0].chapters, [1]);

        assert_eq!(refs[1].book, "Исх");
        assert_eq!(refs[1].locations[0].chapters, [1]);
        assert_eq!(refs[1].locations[0].verses, Some(vec![2, 4]));

        assert_eq!(refs[2].book, "1 Пет");
        assert_eq!(refs[2].locations[0].chapters, [5, 6, 7, 8, 10]);
        assert_eq!(refs[2].locations[0].verses, None);

        assert_eq!(refs[3].book, "Rev");
        assert_eq!(refs[3].locations[0].chapters, [2, 4]);
        assert_eq!(refs[3].locations[0].verses, None);

        assert_eq!(refs[4].book, "Jh");
        assert_eq!(refs[4].locations[0].chapters, [1]);
        assert_eq!(refs[4].locations[0].verses, Some(vec![2, 3, 4, 7]));

        assert_eq!(refs[5].book, "Gen");
        assert_eq!(refs[5].locations[0].chapters, [1]);
        assert_eq!(refs[5].locations[0].verses, Some(vec![1, 2]));
        assert_eq!(refs[5].locations[1].chapters, [2]);
        assert_eq!(refs[5].locations[1].verses, Some(vec![2, 5]));
    }
}
