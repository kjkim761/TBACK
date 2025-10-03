use chrono::{TimeDelta, Utc};
use dioxus::{html::h1, prelude::*};
use rand::Rng;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut max_interval = use_signal(||TimeDelta::zero());
    let mut max_interval_as_seconds = max_interval().as_seconds_f64();
    let mut location = use_signal(|| rand::rng().random_range(1..=3));
    let mut phase = use_signal(|| 0);
    let mut countdown = use_signal(|| 0);
    let mut memo = use_signal(|| Vec::<i32>::with_capacity(30));
    let mut now = use_signal(||Utc::now());
    rsx! {
        h1 { "TBACK" }
        a { href: "https://github.com/kjkim761/TBACK", "<about>" }

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
                    h1 {color:"white", "◯" }
                    h1 { color:"white", "◯" }
                }
            } else if location() == 2 {
                div {
                    h1 { color:"white", "◯" }
                    h1 { "③" }
                    h1 { color:"white", "◯" }
                }
            } else if location() == 3 {
                div {
                    h1 { color:"white", "◯" }
                    h1 { color:"white", "◯" }
                    h1 { "③" }
                }
            }
        } else if countdown() == 2 {
            if location() == 1 {
                div {
                    h1 { "②" }
                    h1 { color:"white", "◯" }
                    h1 { color:"white", "◯" }
                }
            } else if location() == 2 {
                div {
                    h1 { color:"white", "◯" }
                    h1 { "②" }
                    h1 { color:"white", "◯" }
                }
            } else if location() == 3 {
                div {
                    h1 { color:"white", "◯" }
                    h1 { color:"white", "◯" }
                    h1 { "②" }
                }
            }
        } else if countdown() == 1 {
            if location() == 1 {
                div {
                    h1 { "①" }
                    h1 { color:"white", "◯" }
                    h1 { color:"white", "◯" }
                }
            } else if location() == 2 {
                div {
                    h1 { color:"white", "◯" }
                    h1 { "①" }
                    h1 { color:"white", "◯" }
                }
            } else if location() == 3 {
                div {
                    h1 { color:"white", "◯" }
                    h1 { color:"white", "◯" }
                    h1 { "①" }
                }
            }
        }
        if phase() == 0 {
            button {
                onclick: move |ev| async move {
                    phase.set(1);
                    countdown.set(3);
                    location.set(rand::rng().random_range(1..=3));
                    memo.with_mut(|m| m.push(location()));
                    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
                    countdown.set(2);
                    location.set(rand::rng().random_range(1..=3));
                    memo.with_mut(|m| m.push(location()));
                    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
                    countdown.set(1);
                    location.set(rand::rng().random_range(1..=3));
                    memo.with_mut(|m| m.push(location()));
                    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
                    phase.set(2);
                    location.set(rand::rng().random_range(1..=3));
                    memo.with_mut(|m| m.push(location()));
                    now.set(Utc::now());
                },
                "Start"
            }
        } else if phase() == 2 {
            button {
                onclick: move |ev| {
                    if *memo().last().unwrap() == memo()[memo().len() - 4] {
                        let interval = Utc::now() - now();
                        now.set(Utc::now());
                        if interval > max_interval() {
                            max_interval.set(interval);
                        }
                        if memo().len() >= 23 {
                            phase.set(4)
                        }
                        location.set(rand::rng().random_range(1..=3));
                        memo.with_mut(|m| m.push(location()));
                    } else {
                        phase.set(3);
                    }
                },
                "Match"
            }
            button {
                onclick: move |ev| {
                    if *memo().last().unwrap() != memo()[memo().len() - 4] {
                        let interval = Utc::now() - now();
                        now.set(Utc::now());
                        if interval > max_interval() {
                            max_interval.set(interval);
                        }
                        if memo().len() >= 23 {
                            phase.set(4)
                        }
                        location.set(rand::rng().random_range(1..=3));
                        memo.with_mut(|m| m.push(location()));
                    } else {
                        phase.set(3);
                    }
                },
                "Unmatch"
            }
        } else if phase() == 3 {
            h1 { "Wrong!" }
            a { href: "./", "Restart" }
        } else if phase() == 4 {
            h1 { "Clear! Score: {max_interval_as_seconds:.2}" }
            a { href: "./", "Restart" }
        }
    }
}
