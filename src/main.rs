use serde::Deserialize;
#[derive(Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Ip {
    origin: String,
}

struct Context {
    hold_values: Vec<Ip>,
}
struct ContextIterator<'a> {
    iter: std::slice::Iter<'a, Ip>,
}
impl<'a> Context {
    fn new() -> Self {
        Self {
            hold_values: Vec::new(),
        }
    }
    fn add(&mut self, value: Ip) {
        self.hold_values.push(value);
    }
    fn iter(&'a self) -> ContextIterator<'a> {
        ContextIterator {
            iter: self.hold_values.iter(),
        }
    }
}
impl<'a> Iterator for ContextIterator<'a> {
    type Item = Ip;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().cloned()
    }
}

async fn run() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    let mut context = Context::new();
    let ip = reqwest::get("http://httpbin.org/ip")
        .await
        .unwrap()
        .json::<Ip>()
        .await
        .unwrap();
    context.add(ip);

    for ip in context.iter() {
        log::debug!("ip.origin: {:?}", ip.origin);
    }
}

fn main() {
    wasm_bindgen_futures::spawn_local(run());
}

use wasm_bindgen_test::wasm_bindgen_test;
use wasm_bindgen_test::wasm_bindgen_test_configure;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn first_test() {
    run().await;
}
