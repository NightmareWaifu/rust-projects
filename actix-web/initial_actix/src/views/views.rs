//views are here

use render::{rsx, SimpleElement, html, raw};


//limitations: as of current knowledge, can also be used for rendering and no IO

pub fn test_html() -> String{
    let render_rsx = rsx! {
        <div>
          <h1>{"RENDER HEADER"}</h1>
          <p>{"RSX CRATE WORKS WITH ACTIX WEB"}</p>
        </div>
    };
    let render_html = html! {
        <div>
            <p>{"Rendering Raw HTML with render create"}</p>
            <label>{"This is a label with input"}{raw!("<br><input type=text />")}</label>
        </div>
      };
    return render_html;
}