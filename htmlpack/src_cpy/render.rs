/*
* render.rs
*
* fix the \n 
*/

// standard
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::path::{PathBuf};
use std::error::Error;
// external
use maud::{DOCTYPE, html, Markup, PreEscaped};
// local
use crate::html::*;
    
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
        style { "\n"(head.css)"\n" }
        "\n"
    }
}

fn render_metadata(metadata: HtmlMetadata) -> Markup {
    // could only figure out how to make viewport like this with concat?
    // should we let the user customize this?
    let viewport = concat!(
        "width=device-width, ",
        "initial-scale=1.0, ",
        "maximum-scale=1.0, ",
        "user-scalable=1"
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
                type="image/(favicon.mime_type)" 
                href=(format!(
                        "data:image/{};{},{}", 
                        favicon.mime_type,
                        favicon.encoding,
                        favicon.text
                    ));
            //link rel="icon" type="image/svg+xml" href=(format!("data:image/svg+xml;base64,{}", favicons[0]));
            "\n"
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
    // save our string to an html file
    pub fn save_to_file(
        self,
        output: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let html = self.html;
        // create the directory and all parent directories if they don't exist
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent)?;
        }
        //let mut file = File::create(output_dir.join("index.html"))?;
        let mut file = File::create(output)?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }
    
    // API? to display the string?
    // maybe a return as json? what else is possible or useful?
    // pub fn show(self) -> Result<() Box<dyn Error>> { todo!() }
}

