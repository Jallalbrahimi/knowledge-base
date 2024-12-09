pub mod indexer_lib {

    use mdbook::book::{Book, BookItem, Chapter};
    use mdbook::errors::Error;
    use mdbook::preprocess::{Preprocessor, PreprocessorContext};
    use std::collections::HashMap;
    use std::path::PathBuf;

    pub struct Indexer;

    impl Indexer {
        pub fn new() -> Self {
            Indexer
        }
    }

    impl Preprocessor for Indexer {
        fn name(&self) -> &str {
            "indexer_preprocessor"
        }

        fn run(&self, _ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
            let mut updated_book = book.clone();

            let (mentions, tags) = collect_mentions_and_tags(&mut updated_book);

            // Generate index chapters
            add_index_chapter(&mut updated_book, "tags.md", "Tags", "#", &tags);
            add_index_chapter(&mut updated_book, "mentions.md", "Mentions", "@", &mentions);

            Ok(updated_book)
        }

        fn supports_renderer(&self, renderer: &str) -> bool {
            renderer != "not-supported"
        }
    }

    fn collect_mentions_and_tags(
        book: &mut Book,
    ) -> (HashMap<String, Vec<String>>, HashMap<String, Vec<String>>) {
        let mut mentions = HashMap::new();
        let mut tags = HashMap::new();

        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                if let Some(content) = process_chapter(chapter, &mut mentions, &mut tags) {
                    chapter.content = content;
                }
            }
        });

        (mentions, tags)
    }

    fn process_chapter(
        chapter: &mut Chapter,
        mentions: &mut HashMap<String, Vec<String>>,
        tags: &mut HashMap<String, Vec<String>>,
    ) -> Option<String> {
        let mut content = chapter.content.clone();

        // Process tags
        for tag in extract_prefix_items(&content, '#') {
            let tag_link = format!("[#{}](tags.md#{})", tag, tag);
            content = content.replace(&format!("#{}", tag), &tag_link);

            let chapter_path = chapter_path(chapter);
            tags.entry(tag).or_default().push(chapter_path);
        }

        // Process mentions
        for mention in extract_prefix_items(&content, '@') {
            let mention_link = format!("[@{}](mentions.md#{})", mention, mention);
            content = content.replace(&format!("@{}", mention), &mention_link);

            let chapter_path = chapter_path(chapter);
            mentions.entry(mention).or_default().push(chapter_path);
        }

        Some(content)
    }

    fn extract_prefix_items(text: &str, prefix: char) -> Vec<String> {
        text.split(|c: char| c.is_whitespace() || (c != prefix && c.is_ascii_punctuation()))
            .filter_map(|word| {
                word.strip_prefix(prefix)
                    .filter(|&trimmed| !trimmed.is_empty())
                    .map(String::from)
            })
            .collect()
    }

    fn generate_index(title: &str, prefix: &str, index: &HashMap<String, Vec<String>>) -> String {
        index
            .iter()
            .map(|(key, pages)| {
                let entries = pages
                    .iter()
                    .map(|page| format!("- [{}]({})", page, page))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("## {}{}\n{}\n", prefix, key, entries)
            })
            .fold(format!("# {}\n\n", title), |mut md, section| {
                md.push_str(&section);
                md
            })
    }

    fn add_index_chapter(
        book: &mut Book,
        path: &str,
        title: &str,
        prefix: &str,
        index: &HashMap<String, Vec<String>>,
    ) {
        let content = generate_index(title, prefix, index);
        book.sections.push(BookItem::Chapter(Chapter::new(
            path,
            content,
            PathBuf::from(path),
            Vec::new(),
        )));
    }

    fn chapter_path(chapter: &Chapter) -> String {
        chapter
            .path
            .clone()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }
}
