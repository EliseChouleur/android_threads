use android_logger::Config;
use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use log::debug;

pub(crate) static APP_INITIALIZED: std::sync::Once = std::sync::Once::new();

#[no_mangle]
#[allow(non_snake_case)]
pub fn Java_com_example_myapplication_JniInterface_runRustExample(_env: JNIEnv,
                                                                      _class: JClass,
                                                                      _context: JObject,) {
    APP_INITIALIZED.call_once(|| {
        android_logger::init_once(
            Config::default()
                .with_min_level(log::Level::Debug)
                .with_tag("RUST_LIB"),
        );
    });
    debug!("RUST start");_
}