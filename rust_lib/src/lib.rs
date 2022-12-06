use android_logger::Config;
use jni::objects::{JClass, JObject};
use jni::JNIEnv;
use log::debug;

pub(crate) static APP_INITIALIZED: std::sync::Once = std::sync::Once::new();

#[no_mangle]
#[allow(non_snake_case)]
pub fn Java_com_example_myapplication_JniInterface_runRustExample(
    env: JNIEnv,
    _class: JClass,
    _context: JObject,
) {
    APP_INITIALIZED.call_once(|| {
        android_logger::init_once(
            Config::default()
                .with_min_level(log::Level::Debug)
                .with_tag("RUST_LIB"),
        );
    });
    debug!("RUST start");

    let ble_class = env
        .find_class("com/example/myapplication/BleInterface")
        .unwrap();
    let met_call = env.call_static_method(ble_class, "javaTest", "()V", &[]);
    debug!("Method call: {:?}", met_call);
}
