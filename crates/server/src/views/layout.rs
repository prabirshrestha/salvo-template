use crate::routes::assets;

markup::define! {
    Layout<Main: markup::Render>(main: Main) {
        @markup::doctype()
        html {
            head {
                title { "App" }
                link[rel="stylesheet", href=assets::styles_css_href()] {}
            }
            body {
                main {
                    @main
                }
            }
        }
    }
}
