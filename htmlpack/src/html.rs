/*
* html.rs
*
* The data primitive that sits in the middle of the data transformation pipeline.
*/


// standard
// none
// external
// none
// local
use crate::render::{PackedHtml, render_to_packed};

//<!-- For older browsers that don't support WASM natively -->
//<script src="https://cdn.jsdelivr.net/npm/wasm-polyfill/wasm-polyfill.min.js"></script>
// hmmm

// one concern is the use of \n everywhere

// the unpacked document
#[derive(Debug)]
pub struct HtmlDoc {
    pub head: HtmlHead,
    pub body: HtmlBody,
}

#[derive(Debug)]
pub struct HtmlHead {
    pub metadata: HtmlMetadata,
    // assets
    pub favicon: Vec<EncodedIcon>,
    pub css: String, // flatten the css
}

#[derive(Debug)]
pub struct HtmlMetadata {
    // metadata, if not provided just ""
    pub title: String, 
    pub author: String, 
    pub description: String,
    pub keywords: String,
}

#[derive(Debug)]
pub struct HtmlBody {
    // binary assets
    pub encoded_wasm: Vec<EncodedWasm>,
    // javascript no modules
    pub js_scripts: Vec<String>,
    // html snippets
    pub html_shards: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EncodedWasm {
    pub id: String, // identifier
    pub hash: String, //sha-256 hash of the text as bytes
    pub text: String, // text content
}

impl EncodedWasm {
    ///
    /// # Errors
    ///
    /// This function is infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// let wasm = EncodedWasm::new("bin-wasm-app".into(), hash_hex, base64_text);
    /// ```
    pub fn new(id: String, hash: String, text: String) -> Self {
        Self {
            id,
            hash,
            text,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EncodedIcon {
    pub mime_type: String,
    pub encoding: String,
    pub text: String,
}

impl EncodedIcon {
    ///
    /// # Errors
    ///
    /// This function is infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// let icon = EncodedIcon::new("svg+xml".into(), "base64".into(), encoded_svg);
    /// ```
    pub fn new(mime_type: String, encoding: String, text: String) -> Self {
        Self {
            mime_type,
            encoding,
            text,
        }
    }
}

impl HtmlDoc {
    /// This is just a very simple constructor for now
    ///
    /// Might add "builder pattern" later but im not one for that type of thing
    ///
    /// # Errors
    ///
    /// This function is infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// let doc = HtmlDoc::new(
    ///     "My App".into(), "Author".into(), "desc".into(), "kw".into(),
    ///     vec![], String::new(), vec![], vec![], vec![],
    /// );
    /// ```
        pub fn new(
        // head
        title: String,
        author: String,
        description: String,
        keywords: String,
        favicon: Vec<EncodedIcon>,
        css: String,
        // body
        encoded_wasm: Vec<EncodedWasm>,
        js_scripts: Vec<String>,
        html_shards: Vec<String>,
    ) -> Self { 
        Self {
            head: HtmlHead {
                metadata: HtmlMetadata {
                    title,
                    author,
                    description,
                    keywords,
                },
                favicon,
                css,
            },
            body: HtmlBody {
                encoded_wasm,
                js_scripts,
                html_shards
            },
        }
    }

    //pub fn from_config() -> Self { todo!() }

    ///
    /// # Errors
    ///
    /// This function is infallible.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let packed = doc.render();
    /// ```
    // head + body combine into final html page
    pub fn render(self) -> PackedHtml {
        render_to_packed(self)
    }

    ///
    /// # Errors
    ///
    /// This function is infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// let doc = HtmlDoc::empty();
    /// ```
    // need empty htmldoc
    pub fn empty() -> Self {
        Self::new(
            "histos".into(),
            "".into(),
            "".into(),
            "".into(),
            vec![],
            "".into(),
            vec![],
            vec![],
            vec![],
        )
    }

    /*
    impl Into<String>
    this allows you to accept &str or String
    which makes API cleaner
    */

    // metadata builders
    ///
    /// # Errors
    ///
    /// This function is infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// let doc = HtmlDoc::empty().with_title("My App");
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.head.metadata.title = title.into();
        self
    }

    ///
    /// # Errors
    ///
    /// This function is infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// let doc = HtmlDoc::empty().with_author("Jane");
    /// ```
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.head.metadata.author = author.into();
        self
    }

    ///
    /// # Errors
    ///
    /// This function is infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// let doc = HtmlDoc::empty().with_description("A short description");
    /// ```
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.head.metadata.description = description.into();
        self
    }
    
    ///
    /// # Errors
    ///
    /// This function is infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// let doc = HtmlDoc::empty().with_keywords("rust, wasm, app");
    /// ```
    pub fn with_keywords(mut self, keywords: impl Into<String>) -> Self {
        self.head.metadata.keywords = keywords.into();
        self
    }

    // asset builders
    ///
    /// # Errors
    ///
    /// This function is infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// let doc = HtmlDoc::empty().add_html("<div>hello</div>");
    /// ```
    pub fn add_html(mut self, shard: impl Into<String>) -> Self {
        self.body.html_shards.push(shard.into());
        self
    } 

    ///
    /// # Errors
    ///
    /// This function is infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// let doc = HtmlDoc::empty().add_css("body { margin: 0; }");
    /// ```
    pub fn add_css(mut self, css: impl Into<String>) -> Self {
        self.head.css.push_str(&css.into());
        self.head.css.push('\n');
        self
    } 
    
    ///
    /// # Errors
    ///
    /// This function is infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// let doc = HtmlDoc::empty().add_script("console.log('hello')");
    /// ```
    pub fn add_script(mut self, script: impl Into<String>) -> Self {
        self.body.js_scripts.push(script.into());
        self
    } 
    
    ///
    /// # Errors
    ///
    /// This function is infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// let icon = EncodedIcon::new("svg+xml".into(), "base64".into(), encoded_svg);
    /// let doc = HtmlDoc::empty().add_favicon(icon);
    /// ```
    pub fn add_favicon(mut self, favicon: EncodedIcon) -> Self {
        self.head.favicon.push(favicon);
        self
    } 
    
    ///
    /// # Errors
    ///
    /// This function is infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// let wasm = EncodedWasm::new("bin-wasm-app".into(), hash_hex, base64_text);
    /// let doc = HtmlDoc::empty().add_wasm(wasm);
    /// ```
    pub fn add_wasm(mut self, wasm: EncodedWasm) -> Self {
        self.body.encoded_wasm.push(wasm);
        self
    } 
}

