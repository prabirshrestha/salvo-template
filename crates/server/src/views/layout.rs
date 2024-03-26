markup::define! {
    Layout<Main: markup::Render>(main: Main) {
        @markup::doctype()
        html {
            head {
                title { "App" }
            }
            body {
                main {
                    @main
                }
            }
        }
    }
}
