use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            class: "container",
            h1 {
                class: "title my-4 is-1 has-text-centered",
                "Shop",
            },
            (0..3).map(|_| rsx! {
                div {
                    class: "tile is-ancestor",
                    (0..3).map(|_| rsx! {
                        div {
                            class: "tile is-parent",
                            article {
                                class: "tile is-child box",
                                p {
                                    class: "title",
                                    "Cat",
                                },
                                p {
                                    class: "subtitle",
                                    "$25 each",
                                },
                                picture {
                                    class: "image is-square",
                                    source {
                                        src: "/shop/images/cat.avif",
                                        "type": "image/avif",
                                    },
                                    img {
                                        src: "/images/cat.avif",
                                    },
                                },
                                button {
                                    class: "button title is-5 mt-4 is-fullwidth",
                                    "Add to cart",
                                },
                            },
                        },
                    }),
                },
            }),
        },
    })
}
