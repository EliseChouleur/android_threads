#[cfg(feature = "with_robusta")]
mod robusta_java_itf;

#[cfg(feature = "with_robusta")]
use crate::robusta_java_itf::jni::JavaInterface;
use android_logger::Config;
use jni::objects::{GlobalRef, JClass, JObject};
use jni::{JNIEnv, JavaVM};
use log::debug;
use once_cell::sync::OnceCell;
use std::thread;

pub(crate) static APP_INITIALIZED: std::sync::Once = std::sync::Once::new();
static APP_CONTEXT: OnceCell<(JavaVM, GlobalRef, GlobalRef)> = OnceCell::new();

#[no_mangle]
#[allow(non_snake_case)]
pub fn Java_com_example_jnitestapplication_JniInterface_runRustExample(
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

        /* Save class reference */
        let java_class = env
            .find_class("com/example/jnitestapplication/JavaInterface")
            .unwrap();
        let java_class_ref = env.new_global_ref(java_class).unwrap();
        let _ = APP_CONTEXT.set((
            env.get_java_vm().unwrap(),
            env.new_global_ref(app_context).unwrap(),
            java_class_ref,
        ));
    });
    debug!("RUST start");

    let thread_handler = thread::Builder::new()
        .name("communication_thread".to_string())
        .spawn(move || communication_handler());
    let join_res = thread_handler.unwrap().join();
    debug!("Thread ended with success: {}", join_res.is_ok());
}

pub(crate) fn get_app_jni_context(
) -> Result<(JNIEnv<'static>, JObject<'static>, &'static GlobalRef), String> {
    APP_CONTEXT.get().map_or(
        Err("Coudln't get APP_CONTEXT".to_string()),
        |(app_vm, app_context_ref, java_class_ref)| {
            Ok((
                app_vm
                    .attach_current_thread_permanently()
                    .map_err(|e| format!("Couldn't attach thread: {:?}", e))?,
                app_context_ref.as_obj(),
                java_class_ref,
            ))
        },
    )
}

#[cfg(not(feature = "with_robusta"))]
fn communication_handler() {
    debug!("COM_THREAD: Communication handler start...");

    /* Get jni env and app context */
    let get_context = get_app_jni_context();
    debug!(
        "COM_THREAD: Get context success ? {:?}",
        get_context.as_ref().err()
    );
    let (env, _, java_class_ref) = get_context.unwrap();

    /* Get Java class */
    debug!("COM_THREAD: class ref: {:?}", java_class_ref);

    /* Call methode */
    let test_string = env.new_string("SUPER TEST").unwrap();
    let met_call = env.call_static_method(
        java_class_ref,
        "javaTest",
        "(Ljava/lang/String;)I",
        &[test_string.into()],
    );
    debug!("COM_THREAD: Method call: {:?}", met_call);
}

#[cfg(feature = "with_robusta")]
fn communication_handler() {
    debug!("COM_THREAD: Communication handler start...");

    /* Get jni env and app context */
    let get_context = get_app_jni_context();
    debug!(
        "COM_THREAD: Get context success ? {:?}",
        get_context.as_ref().err()
    );
    let (env, _, java_class) = get_context.unwrap();

    /* Call methode */
    let met_call = JavaInterface::javaTest(&env, java_class, "SUPER TEST".to_string());
    debug!("COM_THREAD: Method call: {:?}", met_call);
}
