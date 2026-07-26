/*
* render.rs
*
* fix the \n 
*/

// standard
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::path::{Path};
// local
use crate::error::{HistosResult, SaveError};
// external
use maud::{DOCTYPE, html, Markup, PreEscaped};
// local
use crate::html::*;
    
/// The most basic template for an html file. Head and Body.
/// 
/// # Errors
///
/// This function is infallible.
///
/// # Examples
///
/// ```
/// # use histos::render::render_to_packed;
/// # use histos::html::HtmlDoc;
/// let packed = render_to_packed(HtmlDoc::empty());
/// ```
pub fn render_to_packed(doc: HtmlDoc) -> PackedHtml {
    let markup = html! {
        (DOCTYPE)
        html {
            head { (render_head(doc.head)) }
            body { (render_body(doc.body)) }
        }
    };

    PackedHtml { html: markup.into_string() }
}

fn render_head(head: HtmlHead) -> Markup {
    html! {
        "\n"
        (render_metadata(head.metadata))
        "\n"
        (render_favicons(head.favicon))
        "\n"
        (render_apple_icon(head.apple_icon))
        "\n"
        (render_styles(head.css))
        "\n"
    }
}

fn render_metadata(metadata: HtmlMetadata) -> Markup {
    // could only figure out how to make viewport like this with concat?
    // should we let the user customize this?
    let viewport = concat!(
        //"maximum-scale=1.0, ",
        //"user-scalable=1"
        "width=device-width, ",
        "initial-scale=1.0",
    );
    html! {
        "\n"
        //"<!-- metadata -->"
        meta charset = "utf-8";
        "\n"
        meta name = "viewport" content = (viewport);
        "\n"
        title { (metadata.title) }
        "\n"
        meta name = "description" content = (metadata.description);
        "\n"
        meta name = "author" content = (metadata.author);
        "\n"
        meta name = "keywords" content = (metadata.keywords);
        "\n"
    }
}

// need multiple sizes of icons? 
// overcomplicated for the common case but should implement eventually
fn render_favicons(favicons: Vec<EncodedIcon>) -> Markup {
    if favicons.len() > 0 {
        let favicon = &favicons[0];
        html! {
            // modern browsers - svg best option
            "\n"
            link 
                rel="icon" 
                type=(format!("image/{}", favicon.mime_type))
                href=(format!(
                        "data:image/{};{},{}", 
                        favicon.mime_type,
                        favicon.encoding,
                        favicon.text
                    ));
            //link rel="icon" type="image/svg+xml" href=(format!("data:image/svg+xml;base64,{}", favicons[0]));
            // basic, covers most needs
            //link rel="icon" type="image/x-icon" href="data:image/x-icon;base64,YOUR_ICO_BASE64_HERE";
            // fallback pngs for various sizes
            // why not .ico?
            //link rel="icon" type="image/png" sizes="16x16" href="data:image/png;base64,YOUR_16x16_PNG_BASE64_HERE";
            //link rel="icon" type="image/png" sizes="32x32" href="data:image/png;base64,YOUR_32x32_PNG_BASE64_HERE";
            // apple support
            //link rel="apple-touch-icon" sizes="180x180" href="data:image/png;base64,YOUR_180x180_PNG_BASE64_HERE";
        }
    } else {
        html! {
            "\n"
        }
    }
}

fn render_apple_icon(apple_icon: Vec<EncodedIcon>) -> Markup {
    if apple_icon.len() > 0 {
        let apple_icon = &apple_icon[0];
        html! {
            "\n"
            // apple touch icon for ios homescreen
            link 
                rel="apple-touch-icon" 
                type=(format!("image/{}", apple_icon.mime_type))
                href=(format!(
                        "data:image/{};{},{}", 
                        apple_icon.mime_type,
                        apple_icon.encoding,
                        apple_icon.text
                    ));
        }
    } else {
        html! {
            "\n"
        }
    }
}

fn render_styles(css: Vec<String>) -> Markup {
    html! {
        @for s in &css {
            "\n"
            style { (PreEscaped(s)) }
            "\n"
        }
    }
}

fn render_body(body: HtmlBody) -> Markup {
    html! {
        "\n"
        (render_encoded_wasm(body.encoded_wasm))
        "\n"
        (render_js_scripts(body.js_scripts))
        "\n"
        (render_html_shards(body.html_shards))
        "\n"
    }

}

/*
PreEscaped does the following:
replace problematic characters
tokens to watch for:
&amp; -> &
&lt; -> <
&gt; -> >
&& getting encoded
</script> appearing in strings
unexpected semicolons from minification
*/

// each of the following three functions is slighly different in its rendering

fn render_encoded_wasm(encoded_wasm: Vec<EncodedWasm>) -> Markup {
    html! {
        @for bin in &encoded_wasm {
            "\n"
            pre id=(bin.id) hash=(bin.hash) style="display: none;" {
                (bin.text)
            }
            "\n"
        }
    }
}

fn render_js_scripts(js_scripts: Vec<String>) -> Markup {
    html! {
        @for script in &js_scripts {
            "\n"
            script {
                (PreEscaped(script))
            }
            "\n"
        }
    }
}

fn render_html_shards(html_shards: Vec<String>) -> Markup {
    html! {
        @for shard in &html_shards {
            "\n"
            (PreEscaped(shard))
            "\n"
        }
    }
}

// the final packed string
pub struct PackedHtml {
    pub html: String
}

impl PackedHtml {
    /// Saves the html string to a given output filepath buffer.
    ///
    /// Will create the parent directories if they were not already created.
    ///
    /// # Errors
    ///
    /// - Returns [`SaveError::CreateDir`] if the output directory cannot be created.
    /// - Returns [`SaveError::CreateFile`] if the output file cannot be created.
    /// - Returns [`SaveError::WriteFile`] if writing to the file fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use histos::render::PackedHtml;
    /// # use std::path::Path;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let packed = PackedHtml { html: String::from("<html></html>") };
    /// packed.save_to_file(Path::new("dist/index.html"))?;
    /// # Ok(())
    /// # }
    /// ```
    // save our string to an html file
    pub fn save_to_file(self, output: &Path) -> HistosResult<()> {
        let html = self.html;
        // create the directory and all parent directories if they don't exist
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent)
                .map_err(|source| SaveError::CreateDir { 
                    path: parent.to_path_buf(), source 
                })?;
        }
        let mut file = File::create(&output)
            .map_err(|source| SaveError::CreateFile { 
                path: output.to_path_buf(), source 
            })?;
        file.write_all(html.as_bytes())
            .map_err(|source| SaveError::WriteFile { 
                path: output.to_path_buf(), source 
            })?;
        println!("saved to file {:#?}", output);

        Ok(())
    }
    
    // API? to display the string?
    // maybe a return as json? what else is possible or useful?
    // pub fn show(self) -> Result<() Box<dyn Error>> { todo!() }
}

