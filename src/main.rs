use dioxus::{html::h1, prelude::*};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut location = use_signal(|| fastrand::i32(1..=3));
    let mut phase = use_signal(|| 0);
    let mut countdown = use_signal(|| 0);
    let mut memo = use_signal(|| Vec::<i32>::with_capacity(30));
    let mut now = use_signal(||std::time::SystemTime::now());
    rsx! {
        h1 { "TBACK" }
        if phase() != 1 {
            div { id: "circles",
                h1 {
                    id: "firsto",
                    color: if phase() == 0 || location() == 1 { "black" } else { "white" },
                    "◯"
                }
                h1 {
                    id: "secondo",
                    color: if phase() == 0 || location() == 2 { "black" } else { "white" },
                    "◯"
                }
                h1 {
                    id: "thirdo",
                    color: if phase() == 0 || location() == 3 { "black" } else { "white" },
                    "◯"
                }
            }
        } else if countdown() == 3 {
            if location() == 1 {
                div {
                    h1 { "③" }
                    h1 { "◯" }
                    h1 { "◯" }
                }
            } else if location() == 2 {
                div {
                    h1 { "◯" }
                    h1 { "③" }
                    h1 { "◯" }
                }
            } else if location() == 3 {
                div {
                    h1 { "◯" }
                    h1 { "◯" }
                    h1 { "③" }
                }
            }
        } else if countdown() == 2 {
            if location() == 1 {
                div {
                    h1 { "②" }
                    h1 { "◯" }
                    h1 { "◯" }
                }
            } else if location() == 2 {
                div {
                    h1 { "◯" }
                    h1 { "②" }
                    h1 { "◯" }
                }
            } else if location() == 3 {
                div {
                    h1 { "◯" }
                    h1 { "◯" }
                    h1 { "②" }
                }
            }
        } else if countdown() == 1 {
            if location() == 1 {
                div {
                    h1 { "①" }
                    h1 { "◯" }
                    h1 { "◯" }
                }
            } else if location() == 2 {
                div {
                    h1 { "◯" }
                    h1 { "①" }
                    h1 { "◯" }
                }
            } else if location() == 3 {
                div {
                    h1 { "◯" }
                    h1 { "◯" }
                    h1 { "①" }
                }
            }
        }
        if phase() == 0 {
            button {
                onclick: move |ev| async move {
                    phase.set(1);
                    countdown.set(3);
                    location.set(fastrand::i32(1..=3));
                    memo.with_mut(|m| m.push(location()));
                    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
                    countdown.set(2);
                    location.set(fastrand::i32(1..=3));
                    memo.with_mut(|m| m.push(location()));
                    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
                    countdown.set(1);
                    location.set(fastrand::i32(1..=3));
                    memo.with_mut(|m| m.push(location()));
                    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
                    phase.set(2);
                    location.set(fastrand::i32(1..=3));
                    memo.with_mut(|m| m.push(location()));
                },
                "Start"
            }
        } else if phase() == 2 {
            button { "Match" }
            button { "Unmatch" }
        }
    }
}
