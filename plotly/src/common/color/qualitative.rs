//! Discrete (qualitative) color sequences from Plotly Express.
//!
//! These mirror `plotly.colors.qualitative` in Plotly.py
//! (<https://plotly.com/python/discrete-color/#color-sequences-in-plotly-express>)
//! and can be used wherever a list of CSS colors is accepted, e.g. marker
//! colors or `layout.colorway`.
//!
//! Colors are stored as uppercase `#RRGGBB` hex strings. Plotly.py stores some
//! sequences as `rgb(...)` triplets; the RGB channel values are identical.
//!
//! Each sequence has a reversed counterpart (`*_R`), equivalent to the Python
//! `*_r` lists (e.g. `PLOTLY_R` ↔ `qualitative.Plotly_r`).
//!
//! # Example
//! ```rust
//! use plotly::color::qualitative;
//!
//! let mut colors = Vec::new();
//! for i in 0..12 {
//!     // Cycles when there are more series than colors
//!     colors.push(qualitative::PLOTLY.get(i).to_string());
//! }
//! assert_eq!(colors[0], "#636EFA");
//! assert_eq!(colors[10], "#636EFA");
//! ```
//!
//! A sequence can also be used as a trace colorway:
//!
//! ```rust
//! use plotly::color::qualitative;
//! use plotly::Layout;
//!
//! let _layout = Layout::new().colorway(qualitative::PLOTLY.as_vec());
//! ```

/// A fixed sequence of CSS colors with modular indexing.
///
/// Inspired by `plotly.express` color sequences (`color_discrete_sequence`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColorSequence {
    colors: &'static [&'static str],
    reversed: bool,
}

impl ColorSequence {
    /// Create a color sequence from a static slice of CSS colors.
    pub const fn new(colors: &'static [&'static str]) -> Self {
        Self {
            colors,
            reversed: false,
        }
    }

    /// Number of colors in the sequence (independent of reversal).
    pub const fn len(&self) -> usize {
        self.colors.len()
    }

    /// Returns `true` if the sequence contains no colors.
    pub const fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Whether this sequence yields colors in reverse declaration order.
    pub const fn is_reversed(&self) -> bool {
        self.reversed
    }

    /// Return a sequence with the same colors yielded in reverse order.
    pub const fn reversed(&self) -> Self {
        Self {
            colors: self.colors,
            reversed: !self.reversed,
        }
    }

    /// Get the color at `index`, wrapping around with modulo arithmetic.
    ///
    /// Indexing follows display order: for a reversed sequence, index `0` is
    /// the last declared color.
    ///
    /// # Panics
    ///
    /// Panics if the sequence is empty.
    pub const fn get(&self, index: usize) -> &'static str {
        let n = self.colors.len();
        assert!(n > 0, "ColorSequence::get on empty sequence");
        let i = index % n;
        if self.reversed {
            self.colors[n - 1 - i]
        } else {
            self.colors[i]
        }
    }

    /// Iterate colors in display order.
    pub fn iter(&self) -> Iter {
        Iter {
            sequence: *self,
            index: 0,
        }
    }

    /// Collect colors in display order into a `Vec` (convenient for APIs
    /// expecting `Vec<impl Color>` / marker color lists).
    pub fn as_vec(&self) -> Vec<&'static str> {
        self.iter().collect()
    }
}

/// An iterator over the colors of a [`ColorSequence`] in display order.
#[derive(Debug, Clone)]
pub struct Iter {
    sequence: ColorSequence,
    index: usize,
}

impl Iterator for Iter {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.sequence.len() {
            return None;
        }
        let color = self.sequence.get(self.index);
        self.index += 1;
        Some(color)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.sequence.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for Iter {}

impl IntoIterator for ColorSequence {
    type Item = &'static str;
    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            sequence: self,
            index: 0,
        }
    }
}

impl IntoIterator for &ColorSequence {
    type Item = &'static str;
    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        (*self).into_iter()
    }
}

const PLOTLY_COLORS: [&str; 10] = [
    "#636EFA", "#EF553B", "#00CC96", "#AB63FA", "#FFA15A", "#19D3F3", "#FF6692", "#B6E880",
    "#FF97FF", "#FECB52",
];
const D3_COLORS: [&str; 10] = [
    "#1F77B4", "#FF7F0E", "#2CA02C", "#D62728", "#9467BD", "#8C564B", "#E377C2", "#7F7F7F",
    "#BCBD22", "#17BECF",
];
const G10_COLORS: [&str; 10] = [
    "#3366CC", "#DC3912", "#FF9900", "#109618", "#990099", "#0099C6", "#DD4477", "#66AA00",
    "#B82E2E", "#316395",
];
const T10_COLORS: [&str; 10] = [
    "#4C78A8", "#F58518", "#E45756", "#72B7B2", "#54A24B", "#EECA3B", "#B279A2", "#FF9DA6",
    "#9D755D", "#BAB0AC",
];
const ALPHABET_COLORS: [&str; 26] = [
    "#AA0DFE", "#3283FE", "#85660D", "#782AB6", "#565656", "#1C8356", "#16FF32", "#F7E1A0",
    "#E2E2E2", "#1CBE4F", "#C4451C", "#DEA0FD", "#FE00FA", "#325A9B", "#FEAF16", "#F8A19F",
    "#90AD1C", "#F6222E", "#1CFFCE", "#2ED9FF", "#B10DA1", "#C075A6", "#FC1CBF", "#B00068",
    "#FBE426", "#FA0087",
];
const DARK24_COLORS: [&str; 24] = [
    "#2E91E5", "#E15F99", "#1CA71C", "#FB0D0D", "#DA16FF", "#222A2A", "#B68100", "#750D86",
    "#EB663B", "#511CFB", "#00A08B", "#FB00D1", "#FC0080", "#B2828D", "#6C7C32", "#778AAE",
    "#862A16", "#A777F1", "#620042", "#1616A7", "#DA60CA", "#6C4516", "#0D2A63", "#AF0038",
];
const LIGHT24_COLORS: [&str; 24] = [
    "#FD3216", "#00FE35", "#6A76FC", "#FED4C4", "#FE00CE", "#0DF9FF", "#F6F926", "#FF9616",
    "#479B55", "#EEA6FB", "#DC587D", "#D626FF", "#6E899C", "#00B5F7", "#B68E00", "#C9FBE5",
    "#FF0092", "#22FFA7", "#E3EE9E", "#86CE00", "#BC7196", "#7E7DCD", "#FC6955", "#E48F72",
];
const SET1_COLORS: [&str; 9] = [
    "#E41A1C", "#377EB8", "#4DAF4A", "#984EA3", "#FF7F00", "#FFFF33", "#A65628", "#F781BF",
    "#999999",
];
const PASTEL1_COLORS: [&str; 9] = [
    "#FBB4AE", "#B3CDE3", "#CCEBC5", "#DECBE4", "#FED9A6", "#FFFFCC", "#E5D8BD", "#FDDAEC",
    "#F2F2F2",
];
const DARK2_COLORS: [&str; 8] = [
    "#1B9E77", "#D95F02", "#7570B3", "#E7298A", "#66A61E", "#E6AB02", "#A6761D", "#666666",
];
const SET2_COLORS: [&str; 8] = [
    "#66C2A5", "#FC8D62", "#8DA0CB", "#E78AC3", "#A6D854", "#FFD92F", "#E5C494", "#B3B3B3",
];
const PASTEL2_COLORS: [&str; 8] = [
    "#B3E2CD", "#FDCDAC", "#CBD5E8", "#F4CAE4", "#E6F5C9", "#FFF2AE", "#F1E2CC", "#CCCCCC",
];
const SET3_COLORS: [&str; 12] = [
    "#8DD3C7", "#FFFFB3", "#BEBADA", "#FB8072", "#80B1D3", "#FDB462", "#B3DE69", "#FCCDE5",
    "#D9D9D9", "#BC80BD", "#CCEBC5", "#FFED6F",
];
const ANTIQUE_COLORS: [&str; 11] = [
    "#855C75", "#D9AF6B", "#AF6458", "#736F4C", "#526A83", "#625377", "#68855C", "#9C9C5E",
    "#A06177", "#8C785D", "#7C7C7C",
];
const BOLD_COLORS: [&str; 11] = [
    "#7F3C8D", "#11A579", "#3969AC", "#F2B701", "#E73F74", "#80BA5A", "#E68310", "#008695",
    "#CF1C90", "#F97B72", "#A5AA99",
];
const PASTEL_COLORS: [&str; 11] = [
    "#66C5CC", "#F6CF71", "#F89C74", "#DCB0F2", "#87C55F", "#9EB9F3", "#FE88B1", "#C9DB74",
    "#8BE0A4", "#B497E7", "#B3B3B3",
];
const PRISM_COLORS: [&str; 11] = [
    "#5F4690", "#1D6996", "#38A6A5", "#0F8554", "#73AF48", "#EDAD08", "#E17C05", "#CC503E",
    "#94346E", "#6F4070", "#666666",
];
const SAFE_COLORS: [&str; 11] = [
    "#88CCEE", "#CC6677", "#DDCC77", "#117733", "#332288", "#AA4499", "#44AA99", "#999933",
    "#882255", "#661100", "#888888",
];
const VIVID_COLORS: [&str; 11] = [
    "#E58606", "#5D69B1", "#52BCA3", "#99C945", "#CC61B0", "#24796C", "#DAA51B", "#2F8AC4",
    "#764E9F", "#ED645A", "#A5AA99",
];

/// `Plotly` color sequence (10 colors).
pub const PLOTLY: ColorSequence = ColorSequence::new(&PLOTLY_COLORS);
/// Reversed `Plotly` (Python `plotly.colors.qualitative.Plotly_r`).
pub const PLOTLY_R: ColorSequence = PLOTLY.reversed();
/// `D3` color sequence (10 colors).
pub const D3: ColorSequence = ColorSequence::new(&D3_COLORS);
/// Reversed `D3` (Python `plotly.colors.qualitative.D3_r`).
pub const D3_R: ColorSequence = D3.reversed();
/// `G10` color sequence (10 colors).
pub const G10: ColorSequence = ColorSequence::new(&G10_COLORS);
/// Reversed `G10` (Python `plotly.colors.qualitative.G10_r`).
pub const G10_R: ColorSequence = G10.reversed();
/// `T10` color sequence (10 colors).
pub const T10: ColorSequence = ColorSequence::new(&T10_COLORS);
/// Reversed `T10` (Python `plotly.colors.qualitative.T10_r`).
pub const T10_R: ColorSequence = T10.reversed();
/// `Alphabet` color sequence (26 colors).
pub const ALPHABET: ColorSequence = ColorSequence::new(&ALPHABET_COLORS);
/// Reversed `Alphabet` (Python `plotly.colors.qualitative.Alphabet_r`).
pub const ALPHABET_R: ColorSequence = ALPHABET.reversed();
/// `Dark24` color sequence (24 colors).
pub const DARK24: ColorSequence = ColorSequence::new(&DARK24_COLORS);
/// Reversed `Dark24` (Python `plotly.colors.qualitative.Dark24_r`).
pub const DARK24_R: ColorSequence = DARK24.reversed();
/// `Light24` color sequence (24 colors).
pub const LIGHT24: ColorSequence = ColorSequence::new(&LIGHT24_COLORS);
/// Reversed `Light24` (Python `plotly.colors.qualitative.Light24_r`).
pub const LIGHT24_R: ColorSequence = LIGHT24.reversed();
/// `Set1` color sequence (9 colors).
pub const SET1: ColorSequence = ColorSequence::new(&SET1_COLORS);
/// Reversed `Set1` (Python `plotly.colors.qualitative.Set1_r`).
pub const SET1_R: ColorSequence = SET1.reversed();
/// `Pastel1` color sequence (9 colors).
pub const PASTEL1: ColorSequence = ColorSequence::new(&PASTEL1_COLORS);
/// Reversed `Pastel1` (Python `plotly.colors.qualitative.Pastel1_r`).
pub const PASTEL1_R: ColorSequence = PASTEL1.reversed();
/// `Dark2` color sequence (8 colors).
pub const DARK2: ColorSequence = ColorSequence::new(&DARK2_COLORS);
/// Reversed `Dark2` (Python `plotly.colors.qualitative.Dark2_r`).
pub const DARK2_R: ColorSequence = DARK2.reversed();
/// `Set2` color sequence (8 colors).
pub const SET2: ColorSequence = ColorSequence::new(&SET2_COLORS);
/// Reversed `Set2` (Python `plotly.colors.qualitative.Set2_r`).
pub const SET2_R: ColorSequence = SET2.reversed();
/// `Pastel2` color sequence (8 colors).
pub const PASTEL2: ColorSequence = ColorSequence::new(&PASTEL2_COLORS);
/// Reversed `Pastel2` (Python `plotly.colors.qualitative.Pastel2_r`).
pub const PASTEL2_R: ColorSequence = PASTEL2.reversed();
/// `Set3` color sequence (12 colors).
pub const SET3: ColorSequence = ColorSequence::new(&SET3_COLORS);
/// Reversed `Set3` (Python `plotly.colors.qualitative.Set3_r`).
pub const SET3_R: ColorSequence = SET3.reversed();
/// `Antique` color sequence (11 colors).
pub const ANTIQUE: ColorSequence = ColorSequence::new(&ANTIQUE_COLORS);
/// Reversed `Antique` (Python `plotly.colors.qualitative.Antique_r`).
pub const ANTIQUE_R: ColorSequence = ANTIQUE.reversed();
/// `Bold` color sequence (11 colors).
pub const BOLD: ColorSequence = ColorSequence::new(&BOLD_COLORS);
/// Reversed `Bold` (Python `plotly.colors.qualitative.Bold_r`).
pub const BOLD_R: ColorSequence = BOLD.reversed();
/// `Pastel` color sequence (11 colors).
pub const PASTEL: ColorSequence = ColorSequence::new(&PASTEL_COLORS);
/// Reversed `Pastel` (Python `plotly.colors.qualitative.Pastel_r`).
pub const PASTEL_R: ColorSequence = PASTEL.reversed();
/// `Prism` color sequence (11 colors).
pub const PRISM: ColorSequence = ColorSequence::new(&PRISM_COLORS);
/// Reversed `Prism` (Python `plotly.colors.qualitative.Prism_r`).
pub const PRISM_R: ColorSequence = PRISM.reversed();
/// `Safe` color sequence (11 colors).
pub const SAFE: ColorSequence = ColorSequence::new(&SAFE_COLORS);
/// Reversed `Safe` (Python `plotly.colors.qualitative.Safe_r`).
pub const SAFE_R: ColorSequence = SAFE.reversed();
/// `Vivid` color sequence (11 colors).
pub const VIVID: ColorSequence = ColorSequence::new(&VIVID_COLORS);
/// Reversed `Vivid` (Python `plotly.colors.qualitative.Vivid_r`).
pub const VIVID_R: ColorSequence = VIVID.reversed();

/// All qualitative sequences as `(python_name, sequence)` pairs, in Plotly.py
/// module order.
pub const ALL: &[(&str, ColorSequence)] = &[
    ("Plotly", PLOTLY),
    ("D3", D3),
    ("G10", G10),
    ("T10", T10),
    ("Alphabet", ALPHABET),
    ("Dark24", DARK24),
    ("Light24", LIGHT24),
    ("Set1", SET1),
    ("Pastel1", PASTEL1),
    ("Dark2", DARK2),
    ("Set2", SET2),
    ("Pastel2", PASTEL2),
    ("Set3", SET3),
    ("Antique", ANTIQUE),
    ("Bold", BOLD),
    ("Pastel", PASTEL),
    ("Prism", PRISM),
    ("Safe", SAFE),
    ("Vivid", VIVID),
];

/// Look up a sequence by its Plotly.py name (e.g. `"Plotly"`, `"Dark24"`).
///
/// Names are matched case-insensitively and correspond to the keys of [`ALL`].
/// Reversed sequences are not listed; use [`ColorSequence::reversed`] instead.
pub fn by_name(name: &str) -> Option<ColorSequence> {
    ALL.iter()
        .find(|(sequence_name, _)| sequence_name.eq_ignore_ascii_case(name))
        .map(|(_, sequence)| *sequence)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lengths_match_python() {
        assert_eq!(PLOTLY.len(), 10);
        assert_eq!(D3.len(), 10);
        assert_eq!(G10.len(), 10);
        assert_eq!(T10.len(), 10);
        assert_eq!(ALPHABET.len(), 26);
        assert_eq!(DARK24.len(), 24);
        assert_eq!(LIGHT24.len(), 24);
        assert_eq!(SET1.len(), 9);
        assert_eq!(PASTEL1.len(), 9);
        assert_eq!(DARK2.len(), 8);
        assert_eq!(SET2.len(), 8);
        assert_eq!(PASTEL2.len(), 8);
        assert_eq!(SET3.len(), 12);
        assert_eq!(ANTIQUE.len(), 11);
        assert_eq!(BOLD.len(), 11);
        assert_eq!(PASTEL.len(), 11);
        assert_eq!(PRISM.len(), 11);
        assert_eq!(SAFE.len(), 11);
        assert_eq!(VIVID.len(), 11);
    }

    #[test]
    fn first_and_last_colors() {
        assert_eq!(PLOTLY.get(0), "#636EFA");
        assert_eq!(PLOTLY.get(9), "#FECB52");
        assert_eq!(D3.get(0), "#1F77B4");
        assert_eq!(ALPHABET.get(0), "#AA0DFE");
        assert_eq!(ALPHABET.get(25), "#FA0087");
        // plotly.py carto.Safe[9] is rgb(102, 17, 0)
        assert_eq!(SAFE.get(9), "#661100");
    }

    #[test]
    fn modular_indexing() {
        assert_eq!(PLOTLY.get(10), PLOTLY.get(0));
        assert_eq!(PLOTLY.get(23), PLOTLY.get(3));
        assert_eq!(PLOTLY_R.get(10), PLOTLY_R.get(0));
    }

    #[test]
    fn reversed_order() {
        assert_eq!(PLOTLY_R.len(), PLOTLY.len());
        assert_eq!(PLOTLY_R.get(0), PLOTLY.get(PLOTLY.len() - 1));
        assert_eq!(PLOTLY_R.get(1), PLOTLY.get(PLOTLY.len() - 2));
        assert_eq!(PLOTLY_R.get(9), PLOTLY.get(0));
        assert!(PLOTLY_R.is_reversed());
        assert!(!PLOTLY.is_reversed());
        // Double reverse is identity
        assert_eq!(PLOTLY.reversed().reversed(), PLOTLY);
    }

    #[test]
    fn all_has_19_forward_entries() {
        assert_eq!(ALL.len(), 19);
        let names: Vec<&str> = ALL.iter().map(|(n, _)| *n).collect();
        assert_eq!(
            names,
            vec![
                "Plotly", "D3", "G10", "T10", "Alphabet", "Dark24", "Light24", "Set1", "Pastel1",
                "Dark2", "Set2", "Pastel2", "Set3", "Antique", "Bold", "Pastel", "Prism", "Safe",
                "Vivid",
            ]
        );
    }

    #[test]
    fn by_name_lookup() {
        assert_eq!(by_name("Plotly"), Some(PLOTLY));
        assert_eq!(by_name("Dark24"), Some(DARK24));
        assert_eq!(by_name("plotly"), Some(PLOTLY));
        assert_eq!(by_name("DARK24"), Some(DARK24));
        assert_eq!(by_name("Nope"), None);
    }

    #[test]
    fn all_colors_are_uppercase_hex() {
        for (name, sequence) in ALL {
            for color in sequence.iter() {
                assert_eq!(color.len(), 7, "malformed color in {}: {}", name, color);
                assert!(
                    color.starts_with('#'),
                    "malformed color in {}: {}",
                    name,
                    color
                );
                assert!(
                    color[1..]
                        .chars()
                        .all(|c| matches!(c, '0'..='9' | 'A'..='F')),
                    "non-uppercase-hex color in {}: {}",
                    name,
                    color
                );
            }
        }
    }

    #[test]
    fn iter_collect_display_order() {
        let forward: Vec<&str> = PLOTLY.iter().collect();
        let back: Vec<&str> = PLOTLY_R.iter().collect();
        assert_eq!(forward.len(), 10);
        assert_eq!(back, forward.iter().copied().rev().collect::<Vec<_>>());
    }

    #[test]
    fn empty_sequence_get_panics() {
        let empty = ColorSequence::new(&[]);
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
        let result = std::panic::catch_unwind(|| empty.get(0));
        assert!(result.is_err());
    }
}
