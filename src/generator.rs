use html::inline_text::{Anchor, Span};
use html::inline_text::builders::AnchorBuilder;
use html::metadata::builders::HeadBuilder;
use html::root::Html;
use html::root::builders::BodyBuilder;
use html::text_content::Division;
use html::text_content::builders::DivisionBuilder;

pub fn compile_facade() -> String {
    let mut html_builder = Html::builder();
    html_builder
        .head(head_builder)
        .body(|body| body_builder(body, BodyType::Facade));
    html_builder.build().to_string()
}

pub fn compile_real_site() -> String {
    let mut html_builder = Html::builder();
    html_builder
        .head(head_builder)
        .body(|body| {
            svg_transition(body);
            body_builder(body, BodyType::Facade);
            body_builder(body, BodyType::RealSite)
            }
            );
    html_builder.build().to_string()
}

fn head_builder(head: &mut HeadBuilder) -> &mut HeadBuilder {
    head
        .title(|title| title.text("One Lucky Mushroom") )
        .link(|link| link
                .rel("stylesheet")
                .href("main.css")
        )
        .meta(|meta | meta
                .charset("utf-8")
        )
        .meta(|meta | meta
                .content("width=device-width, initial-scale=1")
                .name("viewport")
        )
        .meta(|meta | meta
                .content("scale")
                .name("text-scale"))
        .script(|script| script.src("main.js"))
}

fn body_builder(body: &mut BodyBuilder, body_type: BodyType) -> &mut BodyBuilder {
    body
        .division(|div| 
            div
                .class(format!("bg {}", body_type.value()))
                .id(format!("{}", body_type.value()))

                /*== MAIN CONTENT BOX  ==*/
                .division(|div| {
                    div.class(format!("content {}", body_type.value()));

                        
                        /*== HEADING ==*/
                        div.division(|div|
                            div
                                .class("header")
                                .division(|div| 
                                    div
                                        .class("pfp-bounding-box")
                                        .division(|div| 
                                            div.class(format!("pfp {}", body_type.value()))
                                        )
                                )
                                .heading_1(|header| 
                                    header.text("Maddie")
                            )
                        );

                        /*== BIO BOXES ==*/ 
                        div.push(left_box(body_type).heading_2(|h2| h2.text("Maker of Mischief")).build());
                        div.push(right_box(body_type).heading_2(|h2| h2.text("Rust Cultist")).build());
                        div.push(left_box(body_type).heading_3(|h2| h2.text("Third Person Self Referrer")).build());
                        
                        div.push(right_box(body_type).division(|div| {
                            div.heading_2(|h2| h2.text("Web Tech Hater"));
                            
                            for point in ["HTML Apostate", "CSS Cynic", "JS Spurner", "Anti-PWA Prejudiced"] {
                                div.anchor(|a| a.class("one-line").text(format!("- {point}"))).line_break(|br| br);
                            }

                            div
                        }).build());

                        /*== CHALLENGE ==*/
                        let mut challenge_box = left_box(body_type);

                        match body_type {
                            BodyType::Facade => {
                                challenge_box.paragraph(|p| p
                                    .text("There's not much here... <br> I wonder if you could do <br> something about that?")
                                    .style("text-wrap: nowrap")
                                    );
                                challenge_box.push(
                                    code_box(body_type).text("U2V0IGNvb2tpZSAndGhvc2Ugd2hvIGtub3cnIHRvICdtZWVlIDozJw==").build()
                                );}
                            BodyType::RealSite => {
                                challenge_box.paragraph(|p| p.text( "Hey, I knew you <br> could do it!").style("text-wrap: nowrap") );
                            }
                        }

                        div.push(challenge_box.build());
                        

                        div.paragraph(|p|p).paragraph(|p|p);

                        /*== EMAIL ==*/
                        if body_type == BodyType::RealSite {  
                            let mut email = right_box(body_type);
                            
                            email
                                .heading_2(|h2| h2.text("Contact Maddie"))
                                .anchor(|a| a
                                    .class("url")
                                    .text("maddie_bl@protonmail.com")
                                    .href("mailto:maddie_bl@protonmail.com")
                            );
                            div.push(email.build());
                        }

                        /*== FRIEND BOXES ==*/
                        let mut friend_box = left_box(body_type);
                        friend_box
                            .heading_2(|h2| h2.text("My Friends!!!").class("one-line"))
                            .push(friend_button("https://kn4ughty.com/blog/new_button/finalv3.webp", "https://kn4ughty.com", "Kn4ughty").build())
                            .push(friend_button("https://necoarc.dev/assets/necoarc-88x31.png", "https://necoarc.dev", "Neco Arc").build())
                            .push(friend_button("https://carminecassie.com/88x31.png", "https://carminecassie.com", "Carmine Cassie").build())
                            .push(friend_button("https://circulars.dev/circular-88x31.gif", "https://circulars.dev", "Circular").build());

                        div.push(friend_box.build());

                        /*== MY BUTTON BOX ==*/
                        let mut my_box = right_box(body_type);
                        my_box
                            .heading_2(|h2| h2.text("My Button!!").class("one-line"))
                            .text("Please feel free to hotlink this on your own website.");
                        my_box.push(friend_button("files/88x31.gif", "javascript:void(0)", "Maddie").style_attr("display: flex; justify-content: center").build());

                        div.push(my_box.build());
                        
                        div.paragraph(|p|p).paragraph(|p|p);

                        /*== PAWSWAP WEBRING ==*/
                        div.division(|div| {
                            div.class("header");
                            let mut webring_box = content_box(body_type);
                            webring_box.anchor(|a| a
                                .class("url")
                                .href("https://webring.pawswap.space")
                                .text("Pawswap.space Webring")
                            );
                            let mut direction_buttons = Span::builder();
                            direction_buttons
                                .style("display: flex; align-items: center")
                                .anchor(|a| a.class("url").href("https://webring.pawswap.space/previous?source=https://oneluckymushroom.dev").text("<--"))
                                .anchor(|a| a.class("url").href("https://webring.pawswap.space/random").text("random"))
                                .anchor(|a| a.class("url").href("https://webring.pawswap.space/next?source=https://oneluckymushroom.dev").text("-->"));
                            webring_box.push(direction_buttons.build());

                            div.push(webring_box.build())
                        });
                        div
                    }
            )
    )
}
fn friend_button(src: &'static str, href: &'static str, friend_name: &'static str) -> AnchorBuilder {
    let mut a = Anchor::builder();
    a
        .href(href)
        .class("friend-button")
        .image(|img| img
            .src(src)
            .alt(format!("{friend_name}'s 88x31 button"))
    );
    a
}

fn content_box(body_type: BodyType) -> DivisionBuilder {
    let mut div = Division::builder();
    div.class(format!("content {} secondary", body_type.value()));
    div
}
fn left_box(body_type: BodyType) -> DivisionBuilder {
    let mut content_box = content_box(body_type);
    let _built = content_box.build();
    let class = _built.class().unwrap_or_default();
    content_box.class(format!("left-box {class}"));
    content_box
}
fn right_box(body_type: BodyType) -> DivisionBuilder {
    let mut content_box = content_box(body_type);
    let _built = content_box.build();
    let class = _built.class().unwrap_or_default();
    content_box.class(format!("right-box {class}")); 
    content_box
}
fn code_box(body_type: BodyType) -> DivisionBuilder {
    let mut content_box = content_box(body_type);
    let _built = content_box.build();
    let class = _built.class().unwrap_or_default();
    content_box.class(format!("{class} code")); 
    content_box
}
fn svg_transition(body: &mut BodyBuilder) -> &mut BodyBuilder {
    body.text("<svg width=\"0\" height=\"0\" xmlns=\"http://www.w3.org/2000/svg\" ><clipPath id=\"circle-mask\" clipPathUnits=\"objectBoundingBox\"><circle r=\"1\" cx=\"0.5\" cy=\"0.5\" id=\"circle\"/></clipPath></svg>")
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BodyType {
    Facade,
    RealSite
}
impl BodyType {
    fn value(&self) -> &'static str {
        match self {
            Self::Facade => "facade",
            Self::RealSite => "real-site"
        }
    }
}
