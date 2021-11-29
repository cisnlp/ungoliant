use crate::pipelines::oscardoc::types::Document;

use super::Annotate;

/// Header/Footer annotator.
///
/// Checks for short sentences in the beginnning/end of documents, and flags if there's too much.
struct Header {
    header_pctg: f64,
    threshold_pctg: f64,
    min_length: usize,
}

impl Default for Header {
    /// Default values are 20% of the document for the header/footer, flagging if >50% of the sentences are short, and < 100 lines = short sentence.
    fn default() -> Self {
        Self {
            header_pctg: 0.2,
            threshold_pctg: 0.5,
            min_length: 100,
        }
    }
}

impl Annotate for Header {
    fn annotate(&self, doc: &mut Document) {
        let nb_lines = doc.content().lines().count();

        // there could be better ways of casting this.
        let nb_lines_header = (nb_lines as f64 * self.header_pctg).floor();
        let treshold_lines = (nb_lines_header as f64 * self.threshold_pctg).floor() as u64;
        let nb_lines_header = nb_lines_header as usize;

        // iterate over the header, counting short lines
        let short_lines_count = self.count_short_lines(doc.content().lines().take(nb_lines_header));

        // moving the if in the for loop may increase/decrease performance?
        if short_lines_count >= treshold_lines {
            doc.metadata_mut().set_annotation("header".to_string());
        }

        let short_lines_count =
            self.count_short_lines(doc.content().lines().rev().take(nb_lines_header));

        if short_lines_count >= treshold_lines {
            doc.metadata_mut().set_annotation("footer".to_string());
        }
    }
}

impl Header {
    fn new(header_pctg: f64, threshold_pctg: f64, min_length: usize) -> Self {
        Self {
            header_pctg,
            threshold_pctg,
            min_length,
        }
    }

    #[inline]
    fn count_short_lines<'a>(&self, lines: impl Iterator<Item = &'a str>) -> u64 {
        // reset counter
        let mut short_lines_count = 0;

        for line in lines {
            if line.len() < self.min_length {
                short_lines_count += 1;
            }
        }

        short_lines_count
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        pipelines::oscardoc::types::{Document, Metadata},
        transformers::Annotate,
    };

    use super::Header;

    #[test]
    fn lengthy_enough() {
        let annotator = Header::new(0.30, 0.60, 30);
        let text = r"This is a lengthy enough sentence! Or at least I hope :)
        This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
short one but it's ok
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)";

        let mut doc = Document::new(text.to_string(), HashMap::new(), Metadata::default());
        annotator.annotate(&mut doc);
        assert_eq!(doc.metadata().annotation(), None);
    }

    #[test]
    fn test_header() {
        let annotator = Header::new(0.30, 0.60, 30);
        let text = r"This is a lengthy enough sentence! Or at least I hope :)
oop, tiny one here
oop, tiny one here
oop, tiny one here
oop, tiny one here
oop, tiny one here
oop, tiny one here
oop, tiny one here
oop, tiny one here
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
short one but it's ok
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)";

        let mut doc = Document::new(text.to_string(), HashMap::new(), Metadata::default());
        annotator.annotate(&mut doc);
        assert_eq!(
            doc.metadata().annotation(),
            Some(&vec!["header".to_string()])
        );
    }

    #[test]
    fn test_footer() {
        let annotator = Header::new(0.30, 0.60, 30);
        let text = r"This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
short one but it's ok
short one but it's ok
short one but it's ok
This is a lengthy enough sentence! Or at least I hope :)";

        let mut doc = Document::new(text.to_string(), HashMap::new(), Metadata::default());
        annotator.annotate(&mut doc);
        assert_eq!(
            doc.metadata().annotation(),
            Some(&vec!["footer".to_string()])
        );
    }

    #[test]
    fn test_both() {
        let annotator = Header::new(0.30, 0.60, 30);
        let text = r"This is a lengthy enough sentence! Or at least I hope :)
short one but it's ok
short one but it's ok
short one but it's ok
short one but it's ok
short one but it's ok
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
This is a lengthy enough sentence! Or at least I hope :)
short one but it's ok
short one but it's ok
short one but it's ok
This is a lengthy enough sentence! Or at least I hope :)";

        let mut doc = Document::new(text.to_string(), HashMap::new(), Metadata::default());
        annotator.annotate(&mut doc);
        assert_eq!(
            doc.metadata().annotation(),
            Some(&vec!["header".to_string(), "footer".to_string()])
        );
    }
}
