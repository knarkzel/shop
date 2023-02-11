use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

#[derive(Props)]
struct ProductProps<'a> {
    name: &'a str,
    price: usize,
    image: &'a str,
    fallback_image: &'a str,
}

fn Product<'a>(cx: Scope<'a, ProductProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "column is-one-third-desktop",
            article {
                class: "box",
                p {
                    class: "title",
                    "{cx.props.name}",
                },
                p {
                    class: "subtitle",
                    "${cx.props.price} each",
                },
                picture {
                    class: "image is-square",
                    source {
                        "type": "image/webp",
                        "srcset": "{cx.props.image}",
                    },
                    img {
                        src: "{cx.props.fallback_image}",
                    },
                },
                button {
                    class: "button title is-5 mt-4 is-fullwidth",
                    "Add to cart",
                },
            }
        },
    })
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            class: "container ",
            h1 {
                class: "title is-1 has-text-centered mt-2",
                "Shop",
            },
            div {
                class: "columns",
                Product { name: "Cat", price: 25, image: "images/cat.webp", fallback_image: "images/cat.jpg" },
                Product { name: "Dog", price: 50, image: "images/dog.webp", fallback_image: "images/dog.jpg" },
                Product { name: "Fox", price: 75, image: "images/fox.webp", fallback_image: "images/fox.jpg" },
            },
        },
    })
}
