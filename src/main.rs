use log::{debug, error, info, trace, warn};

use serde::Deserialize;
#[derive(Clone, Deserialize)]
struct Ip {
    origin: String,
}

#[derive(Debug, Clone)]
enum Perform<T> {
    None,
    Yet(T),
    Already(T),
}
impl<T> Default for Perform<T> {
    fn default() -> Perform<T> {
        Perform::None
    }
}
impl<T> From<Perform<T>> for Option<T> {
    fn from(from: Perform<T>) -> Option<T> {
        let perform = from;
        match perform {
            Perform::None => Option::None,
            Perform::Yet(_) => Option::None,
            Perform::Already(t) => Some(t),
        }
    }
}

mod space {
    use super::{Ip, Perform};
    use once_cell::sync::Lazy;
    use tokio::sync::Mutex;
    static INSTANCE: Lazy<Mutex<Perform<String>>> = Lazy::new(|| {
        let perform = Perform::None;
        Mutex::new(perform)
    });

    pub async fn wasm_future_init() -> anyhow::Result<()> {
        let mut m = INSTANCE.lock().await;
        *m = Perform::Yet("yet".to_string());
        Ok(())
    }

    pub async fn wasm_future_pull() -> anyhow::Result<()> {
        let response = reqwest::get("http://httpbin.org/ip")
            .await?
            .json::<Ip>()
            .await?;
        let mut m = INSTANCE.lock().await;
        *m = Perform::Already(response.origin);
        Ok(())
    }
    pub async fn wasm_future_get() -> Option<String> {
        let l = INSTANCE.lock().await;
        let m: Perform<String> = l.clone();
        From::from(m)
    }
}

fn run() {
    let fut = || async move {
        assert!(space::wasm_future_get().await.is_none());
        assert!(space::wasm_future_init().await.is_ok());
        assert!(space::wasm_future_get().await.is_none());
        assert!(space::wasm_future_pull().await.is_ok());
        let s = space::wasm_future_get().await;
        assert!(s.is_some());
        debug!("ip.origin: {:?}", s);
    };

    wasm_bindgen_futures::spawn_local(fut());
}

fn main() {
    console_error_panic_hook::set_once();

    wasm_logger::init(wasm_logger::Config::default());

    trace!("some trace log");
    debug!("some debug log");
    info!("some info log");
    warn!("some warn log");
    error!("some error log");

    run()
}

use wasm_bindgen_test::wasm_bindgen_test;
use wasm_bindgen_test::wasm_bindgen_test_configure;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn first_test() {
    console_error_panic_hook::set_once();

    wasm_logger::init(wasm_logger::Config::default());

    trace!("some trace log");
    debug!("some debug log");
    info!("some info log");
    warn!("some warn log");
    error!("some error log");

    run();
    assert!(true);
}
