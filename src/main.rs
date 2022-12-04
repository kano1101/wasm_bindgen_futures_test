use serde::Deserialize;
#[derive(Deserialize)]
struct Ip {
    origin: String,
}

mod space {
    use once_cell::sync::Lazy;
    use tokio::sync::Mutex;

    static INSTANCE: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

    pub async fn pull() -> anyhow::Result<()> {
        use super::Ip;
        let response = reqwest::get("http://httpbin.org/ip")
            .await?
            .json::<Ip>()
            .await?;
        let mut lock = INSTANCE.lock().await;
        *lock = Some(response.origin);
        Ok(())
    }
    pub async fn get() -> Option<String> {
        let lock = INSTANCE.lock().await;
        lock.clone()
    }
}

fn run() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    log::trace!("some trace log");
    log::debug!("some debug log");
    log::info!("some info log");
    log::warn!("some warn log");
    log::error!("some error log");

    let fut = || async move {
        assert!(space::get().await.is_none());
        assert!(space::pull().await.is_ok());
        let origin = space::get().await;
        assert!(origin.is_some());
        log::debug!("ip.origin: {:?}", origin);
    };

    wasm_bindgen_futures::spawn_local(fut());
}

fn main() {
    run();
}

use wasm_bindgen_test::wasm_bindgen_test;
use wasm_bindgen_test::wasm_bindgen_test_configure;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn first_test() {
    run();
}
