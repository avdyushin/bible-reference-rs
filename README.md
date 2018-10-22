# Bible Reference Parser
Extract Bible references from plain text in Rust.

## Usage

```rust
let refs = parse(
    "Daily readings are Быт 1;\
     Исх 1:2,4;\
     1 Пет 1-4, 5.\
     Also take a look in:
     Rev 2,4;\
     John 1:2-4,7\
     Gen 1:1-2 2:2,5",
);
println!(refs);
```

Output:

```rust
[
    BibleReference {
        book: "Быт",
        locations: [VerseLocation { chapters: [1], verses: None }]
    },
    BibleReference {
        book: "Исх",
        locations: [VerseLocation { chapters: [1], verses: Some([2, 4]) }]
    },
    BibleReference {
        book: "1 Пет",
        locations: [VerseLocation { chapters: [1, 2, 3, 4, 5], verses: None }]
    },
    BibleReference {
        book: "Rev",
        locations: [VerseLocation { chapters: [2, 4], verses: None }]
    },
    BibleReference {
        book: "John",
        locations: [VerseLocation { chapters: [1], verses: Some([2, 3, 4, 7]) }]
    },
    BibleReference {
        book: "Gen",
        locations: [
            VerseLocation { chapters: [1], verses: Some([1, 2]) },
            VerseLocation { chapters: [2], verses: Some([2, 5]) }
        ]
    }
]
```

### Notes

This library only parses references without validation of the book name because it's different in each language.
