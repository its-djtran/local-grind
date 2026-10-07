//! Build-time syntax highlighting. Produces class-based HTML (`hl-*` classes) so the app can
//! theme code for dark and light mode with CSS, with no highlighting work at runtime.

use std::sync::OnceLock;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

fn syntaxes() -> &'static SyntaxSet {
    static SET: OnceLock<SyntaxSet> = OnceLock::new();
    SET.get_or_init(SyntaxSet::load_defaults_newlines)
}

/// Maps a fence label or language id to a syntax token.
fn token(lang: &str) -> Option<&'static str> {
    match lang.trim().to_ascii_lowercase().as_str() {
        "python" | "py" | "python3" => Some("py"),
        "javascript" | "js" => Some("js"),
        "cpp" | "c++" | "cc" => Some("cpp"),
        "rust" | "rs" => Some("rs"),
        _ => None,
    }
}

/// Highlighted `<pre><code>` block for `code`; unknown languages are escaped plain text.
pub fn highlight(code: &str, lang: &str) -> String {
    let body = token(lang)
        .and_then(|t| syntaxes().find_syntax_by_extension(t))
        .and_then(|syntax| {
            let mut gen = ClassedHTMLGenerator::new_with_class_style(
                syntax,
                syntaxes(),
                ClassStyle::SpacedPrefixed { prefix: "hl-" },
            );
            for line in LinesWithEndings::from(code) {
                gen.parse_html_for_line_which_includes_newline(line).ok()?;
            }
            Some(gen.finalize())
        })
        .unwrap_or_else(|| crate::escape(code));
    let class = token(lang)
        .map(|_| lang.trim().to_ascii_lowercase())
        .unwrap_or_default();
    format!("<pre class=\"code\"><code class=\"language-{class}\">{body}</code></pre>")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlights_supported_languages() {
        for (lang, code, needle) in [
            ("python", "def f(x):\n    return 'a'\n", "hl-keyword"),
            ("js", "var f = function() { return 1; };\n", "hl-storage"),
            ("cpp", "int main() { return 0; }\n", "hl-storage"),
            ("rust", "fn main() { let x = 1; }\n", "hl-storage"),
        ] {
            let html = highlight(code, lang);
            assert!(
                html.starts_with("<pre class=\"code\"><code class=\"language-"),
                "{html}"
            );
            assert!(html.contains(needle), "{lang}: {html}");
        }
    }

    #[test]
    fn unknown_languages_are_escaped() {
        let html = highlight("<b>&", "brainfuck");
        assert_eq!(
            html,
            "<pre class=\"code\"><code class=\"language-\">&lt;b&gt;&amp;</code></pre>"
        );
    }
}
