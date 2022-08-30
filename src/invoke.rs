use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    pub fn __TAURI_INVOKE__(name: &str, arguments: JsValue) -> JsValue;
}

#[macro_export]
macro_rules! invoke {
    {
        $name:ident -> $ty:ty
        $(, $arg:ident : $val:expr)* $(,)?
    } => {async {
        let args = ::js_sys::Map::new();

        $(
            args.set(&stringify!($arg).into(), &::wasm_bindgen::JsValue::from_serde(&$val).unwrap());
        )*

        let output = $crate::invoke::__TAURI_INVOKE__(
            stringify!($name),
            args.into(),
        );

        let promise = ::js_sys::Promise::from(output);
        let result = ::wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
        result.into_serde::<$ty>().unwrap()
    }};
}
