use serde::Deserialize;
#[derive(Deserialize)]
struct Ip {
    origin: String,
}

mod space {
    use once_cell::sync::Lazy;
    use tokio::sync::Mutex;

    #[derive(Debug, Clone)]
    pub enum Perform<T> {
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
    impl<T> Perform<T> {
        pub fn is_none(&self) -> bool {
            if let Perform::None = self {
                return true;
            }
            return false;
        }
        pub fn is_yet(&self) -> bool {
            if let Perform::Yet(_) = self {
                return true;
            }
            return false;
        }
        pub fn is_already(&self) -> bool {
            if let Perform::Already(_) = self {
                return true;
            }
            return false;
        }
    }

    static INSTANCE: Lazy<Mutex<Vec<Perform<String>>>> = Lazy::new(|| Mutex::new(Vec::new()));

    pub async fn get() -> Perform<String> {
        let lock = INSTANCE.lock().await;
        if let Some(first) = lock.first() {
            return first.clone();
        }
        return Perform::None;
    }
    pub async fn add() -> anyhow::Result<()> {
        let mut lock = INSTANCE.lock().await;
        if lock.len() == 0 {
            lock.push(Perform::Yet("テスト中".to_string()));
        }
        Ok(())
    }
    pub async fn pull() -> anyhow::Result<()> {
        use super::Ip;
        let response = reqwest::get("http://httpbin.org/ip")
            .await?
            .json::<Ip>()
            .await?;
        let mut lock = INSTANCE.lock().await;
        if let Some(first) = lock.first_mut() {
            *first = Perform::Already(response.origin);
        }
        Ok(())
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
        assert!(space::add().await.is_ok());
        assert!(space::get().await.is_yet());
        assert!(space::pull().await.is_ok());
        let origin = space::get().await;
        assert!(origin.is_already());
        log::debug!("ip.origin: {:?}", origin);

        // 当然ですがコメントアウトを外すと、localhost:8080からの実行ではエラーになる
        // しかし自動テストの方ではエラーにならない(処理が到達していないことがわかる)
        // assert!(false);
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
