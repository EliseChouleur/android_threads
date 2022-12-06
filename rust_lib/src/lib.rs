use android_logger::Config;
use jni::objects::{GlobalRef, JClass, JObject};
use jni::{AttachGuard, JNIEnv, JavaVM};
use log::debug;
use once_cell::sync::OnceCell;
use std::thread;

pub(crate) static APP_INITIALIZED: std::sync::Once = std::sync::Once::new();
static APP_CONTEXT: OnceCell<(JavaVM, GlobalRef)> = OnceCell::new();

#[no_mangle]
#[allow(non_snake_case)]
pub fn Java_com_example_myapplication_JniInterface_runRustExample(
    env: JNIEnv,
    _class: JClass,
    app_context: JObject,
) {
    APP_INITIALIZED.call_once(|| {
        android_logger::init_once(
            Config::default()
                .with_min_level(log::Level::Debug)
                .with_tag("RUST_LIB"),
        );

        APP_CONTEXT
            .set((
                env.get_java_vm().unwrap(),
                env.new_global_ref(app_context).unwrap(),
            ))
            .unwrap()
    });
    debug!("RUST start");

    let thread_handler = thread::Builder::new()
        .name("communication_thread".to_string())
        .spawn(move || communication_handler());
    let join_res = thread_handler.unwrap().join();
    debug!("Thread ended with success: {}", join_res.is_ok());
}

pub(crate) fn get_app_jni_context() -> Result<(AttachGuard<'static>, JObject<'static>), String> {
    APP_CONTEXT.get().map_or(
        Err("Coudln't get APP_CONTEXT".to_string()),
        |(app_vm, app_context_ref)| {
            Ok((
                app_vm
                    .attach_current_thread()
                    .map_err(|e| format!("Couldn't attach thread: {:?}", e))?,
                app_context_ref.as_obj(),
            ))
        },
    )
}

fn communication_handler() {
    debug!("COM_THREAD: Communication handler start...");
    let get_context = get_app_jni_context();
    debug!("Get context success ? {:?}", get_context.as_ref().err());
    let (env, _) = get_context.unwrap();
    let ble_class_res = env.find_class("com/example/myapplication/BleInterface");
    debug!("BLe class getter res {:?}", ble_class_res);
    let ble_class = ble_class_res.unwrap();
    let met_call = env.call_static_method(ble_class, "javaTest", "()V", &[]);
    debug!("COM_THREAD: Method call: {:?}", met_call);
}
