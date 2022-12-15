use robusta_jni::bridge;

#[bridge]
pub mod jni {
    use jni::sys::jint;
    use robusta_jni::convert::{IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue};
    use robusta_jni::jni::errors::Result as JniResult;
    use robusta_jni::jni::objects::AutoLocal;
    use robusta_jni::jni::objects::GlobalRef;
    use robusta_jni::jni::JNIEnv;

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(com.example.jnitestapplication)]
    pub struct JavaInterface<'env: 'borrow, 'borrow> {
        // "com/example/jnitestapplication/JavaInterface"
        #[instance]
        pub(crate) raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> JavaInterface<'env, 'borrow> {
        /// "java": Rust to Java call
        pub extern "java" fn javaTest(
            env: &JNIEnv,
            class_ref: &GlobalRef,
            test: String,
        ) -> JniResult<jint> {
        } // public static int javaTest(String test);
    }
}
