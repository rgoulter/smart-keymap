# Chorded Keys

Chords (also known as "combos") are a smart keymap feature where
simultaneously pressing multiple keys results in the behaviour of another
key. e.g. pressing 'qw' keys together might send "Backspace"

In this module,
- [Context] stores the [Chord]s of the keymap.
  - [Chord] is defined in terms of keymap indices.
- [Key] describes a key which is part of a chord
  - This includes its 'passthrough key';
    the behaviour of the key when the chord
    didn't succeed.
    (e.g. 'q' or 'w' for the chord 'qw').
- [PressedKeyState] manages chord resolution.
  - If a timeout event is received for the key:
    - if the PKS does not have a satisfied chord,
      the PKS resolves to "Timed out",
      and behaves as the passthrough key.
    - if the PKS has a satisfied chord,
      the PKS resolves to "Chorded key",
      - and the primary PKS
        (the PKS with lowest index in the chord)
        behaves as the chorded key.
  - When a key press is received for some keymap index:
    - if the pressed key does not belong to any of  the chords related to
       that pressed key,
      the PKS resolves to "Interrupted",
      and behaves as the passthrough key.
    - Otherwise,
       when a key press belonging to the chords related to the pressed key occurs:
      - If a chord is fully satisfied (and there are no overlapping chords),
        the PKS resolves to "Chorded key",
        - and the primary PKS
          (the PKS with lowest index in the chord)
          behaves as the chorded key.
      - If a chord is fully satisfied (and there are overlapping chords),
        then the pressed key state remains pending,
      - Otherwise, the PKS remains pending.
  - When a key release event is received for some keymap index:
    - If the PKS is resolved:
      - as chorded: then the chorded key is released (if the PKS was pressing it),
      - as passthrough key: then the pass through key is released.
    - If the PKS is pending,
      - then the passthrough key is 'tapped'.

# Chord Indices
## JSON

Plain key code:

```rust
use smart_keymap::key::chorded::ChordIndices;
let json = r#"
  [3, 4]
"#;
let expected_chord: ChordIndices = ChordIndices::Chord2(3, 4);
let actual_chord: ChordIndices = serde_json::from_str(json).unwrap();
assert_eq!(expected_chord, actual_chord);
```

