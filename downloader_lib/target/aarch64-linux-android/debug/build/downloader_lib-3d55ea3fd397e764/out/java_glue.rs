#[allow(dead_code)]
mod internal_aliases {
    use super::*;
    pub type JStringOptStr = jstring;
    pub type JOptionalInt = jobject;
    pub type JInteger = jobject;
    pub type JByte = jobject;
    pub type JShort = jobject;
    pub type JFloat = jobject;
    pub type JDouble = jobject;
    pub type JOptionalDouble = jobject;
    pub type JLong = jobject;
    pub type JOptionalLong = jobject;
    #[repr(transparent)]
    pub struct JForeignObjectsArray<T: SwigForeignClass> {
        pub(crate) inner: jobjectArray,
        pub(crate) _marker: ::std::marker::PhantomData<T>,
    }
    pub type JStringPath = jstring;
    pub type JStringObjectsArray = jobjectArray;
}
#[doc = " Default JNI_VERSION"]
const SWIG_JNI_VERSION: jint = JNI_VERSION_1_6 as jint;
#[doc = " Marker for what to cache in JNI_OnLoad"]
#[allow(unused_macros)]
macro_rules! swig_jni_find_class {
    ($ id : ident , $ path : expr) => {
        unsafe { $id }
    };
    ($ id : ident , $ path : expr ,) => {
        unsafe { $id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_method_id {
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr) => {
        unsafe { $global_id }
    };
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ,) => {
        unsafe { $global_id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_static_method_id {
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr) => {
        unsafe { $global_id }
    };
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ,) => {
        unsafe { $global_id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_field_id {
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr) => {
        unsafe { $global_id }
    };
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ,) => {
        unsafe { $global_id }
    };
}
#[allow(unused_macros)]
macro_rules! swig_jni_get_static_field_id {
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr) => {
        unsafe { $global_id }
    };
    ($ global_id : ident , $ class_id : ident , $ name : expr , $ sig : expr ,) => {
        unsafe { $global_id }
    };
}
#[allow(dead_code)]
#[doc = ""]
trait SwigInto<T> {
    fn swig_into(self, env: *mut JNIEnv) -> T;
}
#[allow(dead_code)]
#[doc = ""]
trait SwigFrom<T> {
    fn swig_from(_: T, env: *mut JNIEnv) -> Self;
}
#[allow(unused_macros)]
macro_rules! swig_c_str {
    ($ lit : expr) => {
        concat!($lit, "\0").as_ptr() as *const ::std::os::raw::c_char
    };
}
#[allow(unused_macros)]
macro_rules ! swig_assert_eq_size { ($ x : ty , $ ($ xs : ty) ,+ $ (,) *) => { $ (let _ = :: std :: mem :: transmute ::<$ x , $ xs >;) + } ; }
#[cfg(target_pointer_width = "32")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    (val as u32) as *mut T
}
#[cfg(target_pointer_width = "64")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    val as *mut T
}
#[allow(dead_code)]
pub trait SwigForeignClass {
    type PointedType;
    fn jni_class() -> jclass;
    fn jni_class_pointer_field() -> jfieldID;
    fn box_object(x: Self) -> jlong;
    fn unbox_object(x: jlong) -> Self;
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType>;
}
#[allow(dead_code)]
pub trait SwigForeignCLikeEnum {
    fn as_jint(&self) -> jint;
    #[doc = " # Panics"]
    #[doc = " Panics on error"]
    fn from_jint(_: jint) -> Self;
}
#[allow(dead_code)]
struct JavaCallback {
    java_vm: *mut JavaVM,
    this: jobject,
    methods: Vec<jmethodID>,
}
#[doc = " According to JNI spec it should be safe to"]
#[doc = " pass pointer to JavaVm and jobject (global) across threads"]
unsafe impl Send for JavaCallback {}
#[allow(dead_code)]
struct JniEnvHolder<'a> {
    env: Option<*mut JNIEnv>,
    callback: &'a JavaCallback,
    need_detach: bool,
}
#[allow(dead_code)]
impl<'a> Drop for JniEnvHolder<'a> {
    fn drop(&mut self) {
        if self.need_detach {
            let res = unsafe {
                (**self.callback.java_vm).DetachCurrentThread.unwrap()(self.callback.java_vm)
            };
            if res != 0 {
                log::error!("JniEnvHolder: DetachCurrentThread failed: {}", res);
            }
        }
    }
}
#[allow(dead_code)]
impl JavaCallback {
    fn new(obj: jobject, env: *mut JNIEnv) -> JavaCallback {
        let mut java_vm: *mut JavaVM = ::std::ptr::null_mut();
        let ret = unsafe { (**env).GetJavaVM.unwrap()(env, &mut java_vm) };
        assert_eq!(0, ret, "GetJavaVm failed");
        let global_obj = unsafe { (**env).NewGlobalRef.unwrap()(env, obj) };
        assert!(!global_obj.is_null());
        JavaCallback {
            java_vm,
            this: global_obj,
            methods: Vec::new(),
        }
    }
    fn get_jni_env(&self) -> JniEnvHolder {
        assert!(!self.java_vm.is_null());
        let mut env: *mut JNIEnv = ::std::ptr::null_mut();
        let res = unsafe {
            (**self.java_vm).GetEnv.unwrap()(
                self.java_vm,
                (&mut env) as *mut *mut JNIEnv as *mut *mut ::std::os::raw::c_void,
                SWIG_JNI_VERSION,
            )
        };
        if res == (JNI_OK as jint) {
            return JniEnvHolder {
                env: Some(env),
                callback: self,
                need_detach: false,
            };
        }
        if res != (JNI_EDETACHED as jint) {
            panic!("get_jni_env: GetEnv return error `{}`", res);
        }
        trait ConvertPtr<T> {
            fn convert_ptr(self) -> T;
        }
        impl ConvertPtr<*mut *mut ::std::os::raw::c_void> for *mut *mut JNIEnv {
            fn convert_ptr(self) -> *mut *mut ::std::os::raw::c_void {
                self as *mut *mut ::std::os::raw::c_void
            }
        }
        impl ConvertPtr<*mut *mut JNIEnv> for *mut *mut JNIEnv {
            fn convert_ptr(self) -> *mut *mut JNIEnv {
                self
            }
        }
        let res = unsafe {
            (**self.java_vm).AttachCurrentThread.unwrap()(
                self.java_vm,
                (&mut env as *mut *mut JNIEnv).convert_ptr(),
                ::std::ptr::null_mut(),
            )
        };
        if res != 0 {
            log::error!(
                "JavaCallback::get_jnienv: AttachCurrentThread failed: {}",
                res
            );
            JniEnvHolder {
                env: None,
                callback: self,
                need_detach: false,
            }
        } else {
            assert!(!env.is_null());
            JniEnvHolder {
                env: Some(env),
                callback: self,
                need_detach: true,
            }
        }
    }
}
#[allow(dead_code)]
impl Drop for JavaCallback {
    fn drop(&mut self) {
        let env = self.get_jni_env();
        if let Some(env) = env.env {
            assert!(!env.is_null());
            unsafe { (**env).DeleteGlobalRef.unwrap()(env, self.this) };
        } else {
            log::error!("JavaCallback::drop failed, can not get JNIEnv");
        }
    }
}
#[allow(dead_code)]
fn jni_throw(env: *mut JNIEnv, ex_class: jclass, message: &str) {
    let c_message = ::std::ffi::CString::new(message).unwrap();
    let res = unsafe { (**env).ThrowNew.unwrap()(env, ex_class, c_message.as_ptr()) };
    if res != 0 {
        log::error!(
            "JNI ThrowNew({}) failed for class {:?} failed",
            message,
            ex_class
        );
    }
}
#[allow(dead_code)]
fn jni_throw_exception(env: *mut JNIEnv, message: &str) {
    let exception_class = swig_jni_find_class!(JAVA_LANG_EXCEPTION, "java/lang/Exception");
    jni_throw(env, exception_class, message)
}
#[allow(dead_code)]
fn object_to_jobject<T: SwigForeignClass>(env: *mut JNIEnv, obj: T) -> jobject {
    let jcls = <T>::jni_class();
    assert!(!jcls.is_null());
    let field_id = <T>::jni_class_pointer_field();
    assert!(!field_id.is_null());
    let jobj: jobject = unsafe { (**env).AllocObject.unwrap()(env, jcls) };
    assert!(!jobj.is_null(), "object_to_jobject: AllocObject failed");
    let ret: jlong = <T>::box_object(obj);
    unsafe {
        (**env).SetLongField.unwrap()(env, jobj, field_id, ret);
        if (**env).ExceptionCheck.unwrap()(env) != 0 {
            panic!("object_to_jobject: Can not set mNativeObj field: catch exception");
        }
    }
    jobj
}
#[allow(dead_code)]
fn jobject_array_to_vec_of_objects<T: SwigForeignClass + Clone>(
    env: *mut JNIEnv,
    arr: internal_aliases::JForeignObjectsArray<T>,
) -> Vec<T> {
    let field_id = <T>::jni_class_pointer_field();
    assert!(!field_id.is_null());
    let length = unsafe { (**env).GetArrayLength.unwrap()(env, arr.inner) };
    let len = <usize as ::std::convert::TryFrom<jsize>>::try_from(length)
        .expect("invalid jsize, in jsize => usize conversion");
    let mut result = Vec::with_capacity(len);
    for i in 0..length {
        let native: &mut T = unsafe {
            let obj = (**env).GetObjectArrayElement.unwrap()(env, arr.inner, i);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("Failed to retrieve element {} from this `jobjectArray'", i);
            }
            let ptr = (**env).GetLongField.unwrap()(env, obj, field_id);
            let native = (jlong_to_pointer(ptr) as *mut T).as_mut().unwrap();
            (**env).DeleteLocalRef.unwrap()(env, obj);
            native
        };
        result.push(native.clone());
    }
    result
}
#[allow(dead_code)]
fn vec_of_objects_to_jobject_array<T: SwigForeignClass>(
    env: *mut JNIEnv,
    mut arr: Vec<T>,
) -> internal_aliases::JForeignObjectsArray<T> {
    let jcls: jclass = <T>::jni_class();
    assert!(!jcls.is_null());
    let arr_len = <jsize as ::std::convert::TryFrom<usize>>::try_from(arr.len())
        .expect("invalid usize, in usize => to jsize conversion");
    let obj_arr: jobjectArray =
        unsafe { (**env).NewObjectArray.unwrap()(env, arr_len, jcls, ::std::ptr::null_mut()) };
    assert!(!obj_arr.is_null());
    let field_id = <T>::jni_class_pointer_field();
    assert!(!field_id.is_null());
    for (i, r_obj) in arr.drain(..).enumerate() {
        let jobj: jobject = unsafe { (**env).AllocObject.unwrap()(env, jcls) };
        assert!(!jobj.is_null());
        let r_obj: jlong = <T>::box_object(r_obj);
        unsafe {
            (**env).SetLongField.unwrap()(env, jobj, field_id, r_obj);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("Can not mNativeObj field: catch exception");
            }
            (**env).SetObjectArrayElement.unwrap()(env, obj_arr, i as jsize, jobj);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("SetObjectArrayElement({}) failed", i);
            }
            (**env).DeleteLocalRef.unwrap()(env, jobj);
        }
    }
    internal_aliases::JForeignObjectsArray {
        inner: obj_arr,
        _marker: ::std::marker::PhantomData,
    }
}
#[allow(dead_code)]
trait JniInvalidValue {
    fn jni_invalid_value() -> Self;
}
impl<T> JniInvalidValue for *const T {
    fn jni_invalid_value() -> Self {
        ::std::ptr::null()
    }
}
impl<T> JniInvalidValue for *mut T {
    fn jni_invalid_value() -> Self {
        ::std::ptr::null_mut()
    }
}
impl JniInvalidValue for () {
    fn jni_invalid_value() {}
}
impl<T: SwigForeignClass> JniInvalidValue for internal_aliases::JForeignObjectsArray<T> {
    fn jni_invalid_value() -> Self {
        Self {
            inner: ::std::ptr::null_mut(),
            _marker: ::std::marker::PhantomData,
        }
    }
}
macro_rules ! impl_jni_jni_invalid_value { ($ ($ type : ty) *) => ($ (impl JniInvalidValue for $ type { fn jni_invalid_value () -> Self { <$ type >:: default () } }) *) }
impl_jni_jni_invalid_value! { jbyte jshort jint jlong jfloat jdouble jboolean }
#[allow(dead_code)]
pub fn u64_to_jlong_checked(x: u64) -> jlong {
    <jlong as ::std::convert::TryFrom<u64>>::try_from(x)
        .expect("invalid u64, in u64 => jlong conversion")
}
#[allow(dead_code)]
struct JavaUTF16Slice {
    string: jstring,
    chars: *const ::std::os::raw::c_ushort,
    len: usize,
    env: *mut JNIEnv,
}
#[allow(dead_code)]
impl JavaUTF16Slice {
    pub fn new(env: *mut JNIEnv, js: jstring) -> JavaUTF16Slice {
        let len = unsafe { (**env).GetStringLength.unwrap()(env, js) };
        let chars = unsafe { (**env).GetStringChars.unwrap()(env, js, ::std::ptr::null_mut()) };
        JavaUTF16Slice {
            string: js,
            chars,
            len: len as usize,
            env,
        }
    }
    pub fn as_slice(&self) -> &[u16] {
        unsafe { ::std::slice::from_raw_parts(self.chars, self.len) }
    }
    pub fn into_string(self) -> String {
        String::from_utf16(self.as_slice()).unwrap()
    }
}
#[allow(dead_code)]
impl Drop for JavaUTF16Slice {
    fn drop(&mut self) {
        unsafe {
            (**self.env).ReleaseStringChars.unwrap()(self.env, self.string, self.chars);
        }
    }
}
#[allow(dead_code)]
fn from_jstring_std_string(js: jstring, env: *mut JNIEnv) -> String {
    if !js.is_null() {
        JavaUTF16Slice::new(env, js).into_string()
    } else {
        "".to_string()
    }
}
#[allow(dead_code)]
fn from_std_str_jstring(x: &str, env: *mut JNIEnv) -> jstring {
    let x: Vec<::std::os::raw::c_ushort> = x.encode_utf16().collect();
    unsafe { (**env).NewString.unwrap()(env, x.as_ptr(), x.len() as i32) }
}
#[allow(dead_code)]
fn vec_string_to_jobject_array(mut arr: Vec<String>, env: *mut JNIEnv) -> jobjectArray {
    let jcls: jclass = swig_jni_find_class!(JAVA_LANG_STRING, "java/lang/String");
    assert!(!jcls.is_null());
    let obj_arr: jobjectArray = unsafe {
        (**env).NewObjectArray.unwrap()(env, arr.len() as jsize, jcls, ::std::ptr::null_mut())
    };
    assert!(!obj_arr.is_null());
    for (i, r_str) in arr.drain(..).enumerate() {
        let jstr: jstring = from_std_str_jstring(&r_str, env);
        assert!(!jstr.is_null());
        unsafe {
            (**env).SetObjectArrayElement.unwrap()(env, obj_arr, i as jsize, jstr);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                panic!("SetObjectArrayElement({}) failed", i);
            }
            (**env).DeleteLocalRef.unwrap()(env, jstr);
        }
    }
    obj_arr
}
macro_rules ! define_array_handling_code { ($ ([jni_arr_type = $ jni_arr_type : ident , rust_arr_wrapper = $ rust_arr_wrapper : ident , jni_get_array_elements = $ jni_get_array_elements : ident , jni_elem_type = $ jni_elem_type : ident , rust_elem_type = $ rust_elem_type : ident , jni_release_array_elements = $ jni_release_array_elements : ident , jni_new_array = $ jni_new_array : ident , jni_set_array_region = $ jni_set_array_region : ident]) ,*) => { $ (# [allow (dead_code)] struct $ rust_arr_wrapper { array : $ jni_arr_type , data : * mut $ jni_elem_type , env : * mut JNIEnv , } # [allow (dead_code)] impl $ rust_arr_wrapper { fn new (env : * mut JNIEnv , array : $ jni_arr_type) -> $ rust_arr_wrapper { assert ! (! array . is_null ()) ; let data = unsafe { (** env) .$ jni_get_array_elements . unwrap () (env , array , :: std :: ptr :: null_mut ()) } ; $ rust_arr_wrapper { array , data , env } } fn to_slice (& self) -> & [$ rust_elem_type] { unsafe { let len : jsize = (** self . env) . GetArrayLength . unwrap () (self . env , self . array) ; assert ! ((len as u64) <= (usize :: max_value () as u64)) ; :: std :: slice :: from_raw_parts (self . data , len as usize) } } fn from_slice_to_raw (arr : & [$ rust_elem_type] , env : * mut JNIEnv) -> $ jni_arr_type { assert ! ((arr . len () as u64) <= (jsize :: max_value () as u64)) ; let jarr : $ jni_arr_type = unsafe { (** env) .$ jni_new_array . unwrap () (env , arr . len () as jsize) } ; assert ! (! jarr . is_null ()) ; unsafe { (** env) .$ jni_set_array_region . unwrap () (env , jarr , 0 , arr . len () as jsize , arr . as_ptr ()) ; if (** env) . ExceptionCheck . unwrap () (env) != 0 { panic ! ("{}:{} {} failed" , file ! () , line ! () , stringify ! ($ jni_set_array_region)) ; } } jarr } } # [allow (dead_code)] impl Drop for $ rust_arr_wrapper { fn drop (& mut self) { assert ! (! self . env . is_null ()) ; assert ! (! self . array . is_null ()) ; unsafe { (** self . env) .$ jni_release_array_elements . unwrap () (self . env , self . array , self . data , JNI_ABORT as jint ,) } ; } }) * } }
define_array_handling_code!(
    [
        jni_arr_type = jbyteArray,
        rust_arr_wrapper = JavaByteArray,
        jni_get_array_elements = GetByteArrayElements,
        jni_elem_type = jbyte,
        rust_elem_type = i8,
        jni_release_array_elements = ReleaseByteArrayElements,
        jni_new_array = NewByteArray,
        jni_set_array_region = SetByteArrayRegion
    ],
    [
        jni_arr_type = jshortArray,
        rust_arr_wrapper = JavaShortArray,
        jni_get_array_elements = GetShortArrayElements,
        jni_elem_type = jshort,
        rust_elem_type = i16,
        jni_release_array_elements = ReleaseShortArrayElements,
        jni_new_array = NewShortArray,
        jni_set_array_region = SetShortArrayRegion
    ],
    [
        jni_arr_type = jintArray,
        rust_arr_wrapper = JavaIntArray,
        jni_get_array_elements = GetIntArrayElements,
        jni_elem_type = jint,
        rust_elem_type = i32,
        jni_release_array_elements = ReleaseIntArrayElements,
        jni_new_array = NewIntArray,
        jni_set_array_region = SetIntArrayRegion
    ],
    [
        jni_arr_type = jlongArray,
        rust_arr_wrapper = JavaLongArray,
        jni_get_array_elements = GetLongArrayElements,
        jni_elem_type = jlong,
        rust_elem_type = i64,
        jni_release_array_elements = ReleaseLongArrayElements,
        jni_new_array = NewLongArray,
        jni_set_array_region = SetLongArrayRegion
    ],
    [
        jni_arr_type = jfloatArray,
        rust_arr_wrapper = JavaFloatArray,
        jni_get_array_elements = GetFloatArrayElements,
        jni_elem_type = jfloat,
        rust_elem_type = f32,
        jni_release_array_elements = ReleaseFloatArrayElements,
        jni_new_array = NewFloatArray,
        jni_set_array_region = SetFloatArrayRegion
    ],
    [
        jni_arr_type = jdoubleArray,
        rust_arr_wrapper = JavaDoubleArray,
        jni_get_array_elements = GetDoubleArrayElements,
        jni_elem_type = jdouble,
        rust_elem_type = f64,
        jni_release_array_elements = ReleaseDoubleArrayElements,
        jni_new_array = NewDoubleArray,
        jni_set_array_region = SetDoubleArrayRegion
    ]
);
#[allow(dead_code)]
fn to_java_util_optional_double(
    env: *mut JNIEnv,
    x: Option<f64>,
) -> internal_aliases::JOptionalDouble {
    let class: jclass = swig_jni_find_class!(JAVA_UTIL_OPTIONAL_DOUBLE, "java/util/OptionalDouble");
    assert!(!class.is_null(),);
    match x {
        Some(val) => {
            let of_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_DOUBLE_OF,
                JAVA_UTIL_OPTIONAL_DOUBLE,
                "of",
                "(D)Ljava/util/OptionalDouble;"
            );
            assert!(!of_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, of_m, val);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalDouble.of failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
        None => {
            let empty_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_DOUBLE_EMPTY,
                JAVA_UTIL_OPTIONAL_DOUBLE,
                "empty",
                "()Ljava/util/OptionalDouble;"
            );
            assert!(!empty_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, empty_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalDouble.empty failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_double_to_rust(env: *mut JNIEnv, x: internal_aliases::JDouble) -> Option<f64> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_DOUBLE, "java/lang/Double");
            assert!(!class.is_null());
            let double_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_DOUBLE_DOUBLE_VALUE_METHOD,
                JAVA_LANG_DOUBLE,
                "doubleValue",
                "()D",
            );
            assert!(!double_value_m.is_null(),);
            let ret: f64 = unsafe {
                let ret = (**env).CallDoubleMethod.unwrap()(env, x, double_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Double.doubleValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_float_to_rust(env: *mut JNIEnv, x: internal_aliases::JFloat) -> Option<f32> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_FLOAT, "java/lang/Float");
            assert!(!class.is_null());
            let float_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_FLOAT_FLOAT_VALUE,
                JAVA_LANG_FLOAT,
                "floatValue",
                "()F"
            );
            assert!(!float_value_m.is_null());
            let ret: f32 = unsafe {
                let ret = (**env).CallFloatMethod.unwrap()(env, x, float_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Float.floatValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn to_java_util_optional_long(env: *mut JNIEnv, x: Option<i64>) -> internal_aliases::JOptionalLong {
    let class: jclass = swig_jni_find_class!(JAVA_UTIL_OPTIONAL_LONG, "java/util/OptionalLong");
    assert!(!class.is_null(),);
    match x {
        Some(val) => {
            let of_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_LONG_OF,
                JAVA_UTIL_OPTIONAL_LONG,
                "of",
                "(J)Ljava/util/OptionalLong;"
            );
            assert!(!of_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, of_m, val);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalLong.of failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
        None => {
            let empty_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_LONG_EMPTY,
                JAVA_UTIL_OPTIONAL_LONG,
                "empty",
                "()Ljava/util/OptionalLong;",
            );
            assert!(!empty_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, empty_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalLong.empty failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_long_to_rust(env: *mut JNIEnv, x: internal_aliases::JLong) -> Option<i64> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_LONG, "java/lang/Long");
            assert!(!class.is_null());
            let long_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_LONG_LONG_VALUE,
                JAVA_LANG_LONG,
                "longValue",
                "()J"
            );
            assert!(!long_value_m.is_null());
            let ret: i64 = unsafe {
                let ret = (**env).CallLongMethod.unwrap()(env, x, long_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Long.longValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_int_to_rust(env: *mut JNIEnv, x: internal_aliases::JInteger) -> Option<i32> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_INTEGER, "java/lang/Integer");
            assert!(!class.is_null());
            let int_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_INTEGER_INT_VALUE,
                JAVA_LANG_INTEGER,
                "intValue",
                "()I"
            );
            assert!(!int_value_m.is_null(),);
            let ret: i32 = unsafe {
                let ret = (**env).CallIntMethod.unwrap()(env, x, int_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Integer.intValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_byte_to_rust(env: *mut JNIEnv, x: internal_aliases::JByte) -> Option<i8> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_BYTE, "java/lang/Byte");
            assert!(!class.is_null());
            let byte_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_BYTE_BYTE_VALUE,
                JAVA_LANG_BYTE,
                "byteValue",
                "()B"
            );
            assert!(!byte_value_m.is_null(),);
            let ret: i8 = unsafe {
                let ret = (**env).CallByteMethod.unwrap()(env, x, byte_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Byte.byteValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn from_java_lang_short_to_rust(env: *mut JNIEnv, x: internal_aliases::JByte) -> Option<i16> {
    if x.is_null() {
        None
    } else {
        let x = unsafe { (**env).NewLocalRef.unwrap()(env, x) };
        if x.is_null() {
            None
        } else {
            let class: jclass = swig_jni_find_class!(JAVA_LANG_SHORT, "java/lang/Short");
            assert!(!class.is_null());
            let short_value_m: jmethodID = swig_jni_get_method_id!(
                JAVA_LANG_SHORT_SHORT_VALUE,
                JAVA_LANG_SHORT,
                "shortValue",
                "()S"
            );
            assert!(!short_value_m.is_null());
            let ret: i16 = unsafe {
                let ret = (**env).CallShortMethod.unwrap()(env, x, short_value_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("Short.shortValue failed: catch exception");
                }
                (**env).DeleteLocalRef.unwrap()(env, x);
                ret
            };
            Some(ret)
        }
    }
}
#[allow(dead_code)]
fn to_java_util_optional_int(env: *mut JNIEnv, x: Option<i32>) -> jobject {
    let class: jclass = swig_jni_find_class!(JAVA_UTIL_OPTIONAL_INT, "java/util/OptionalInt");
    assert!(!class.is_null(),);
    match x {
        Some(val) => {
            let of_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_INT_OF,
                JAVA_UTIL_OPTIONAL_INT,
                "of",
                "(I)Ljava/util/OptionalInt;"
            );
            assert!(!of_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, of_m, val);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalInt.of failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
        None => {
            let empty_m: jmethodID = swig_jni_get_static_method_id!(
                JAVA_UTIL_OPTIONAL_INT_EMPTY,
                JAVA_UTIL_OPTIONAL_INT,
                "empty",
                "()Ljava/util/OptionalInt;"
            );
            assert!(!empty_m.is_null());
            let ret = unsafe {
                let ret = (**env).CallStaticObjectMethod.unwrap()(env, class, empty_m);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    panic!("OptionalInt.empty failed: catch exception");
                }
                ret
            };
            assert!(!ret.is_null());
            ret
        }
    }
}
use crate::*;
use jni_sys::*;
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_JNIReachabilityFence_reachabilityFence1(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_JNIReachabilityFence_reachabilityFence2(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_JNIReachabilityFence_reachabilityFence3(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_JNIReachabilityFence_reachabilityFence4(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_JNIReachabilityFence_reachabilityFence5(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_JNIReachabilityFence_reachabilityFence6(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_JNIReachabilityFence_reachabilityFence7(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_JNIReachabilityFence_reachabilityFence8(
    _env: *mut JNIEnv,
    _: jclass,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
    _: jobject,
) {
}
impl SwigForeignCLikeEnum for TypeOfFile {
    fn as_jint(&self) -> jint {
        match *self {
            TypeOfFile::Word => 0i32,
            TypeOfFile::Excel => 1i32,
            TypeOfFile::PowerPoint => 2i32,
            TypeOfFile::Jpg => 3i32,
            TypeOfFile::Mkv => 4i32,
            TypeOfFile::Png => 5i32,
            TypeOfFile::Html => 6i32,
            TypeOfFile::MpFour => 7i32,
            TypeOfFile::Pdf => 8i32,
            TypeOfFile::Other => 9i32,
            TypeOfFile::Application => 10i32,
            TypeOfFile::Audio => 11i32,
            TypeOfFile::MpThree => 12i32,
            TypeOfFile::Gif => 13i32,
            TypeOfFile::Video => 14i32,
            TypeOfFile::Zip => 15i32,
            TypeOfFile::Image => 16i32,
            TypeOfFile::Iso => 17i32,
            TypeOfFile::ThreeGp => 18i32,
            TypeOfFile::Flv => 19i32,
            TypeOfFile::Document => 20i32,
            TypeOfFile::Compressed => 21i32,
        }
    }
    fn from_jint(x: jint) -> Self {
        match x {
            0i32 => TypeOfFile::Word,
            1i32 => TypeOfFile::Excel,
            2i32 => TypeOfFile::PowerPoint,
            3i32 => TypeOfFile::Jpg,
            4i32 => TypeOfFile::Mkv,
            5i32 => TypeOfFile::Png,
            6i32 => TypeOfFile::Html,
            7i32 => TypeOfFile::MpFour,
            8i32 => TypeOfFile::Pdf,
            9i32 => TypeOfFile::Other,
            10i32 => TypeOfFile::Application,
            11i32 => TypeOfFile::Audio,
            12i32 => TypeOfFile::MpThree,
            13i32 => TypeOfFile::Gif,
            14i32 => TypeOfFile::Video,
            15i32 => TypeOfFile::Zip,
            16i32 => TypeOfFile::Image,
            17i32 => TypeOfFile::Iso,
            18i32 => TypeOfFile::ThreeGp,
            19i32 => TypeOfFile::Flv,
            20i32 => TypeOfFile::Document,
            21i32 => TypeOfFile::Compressed,
            _ => panic!(concat!("{} not expected for ", stringify!(TypeOfFile)), x),
        }
    }
}
#[allow(dead_code)]
impl SwigFrom<TypeOfFile> for jobject {
    fn swig_from(x: TypeOfFile, env: *mut JNIEnv) -> jobject {
        let cls: jclass = swig_jni_find_class!(
            FOREIGN_ENUM_TYPEOFFILE,
            "com/example/swift_final/lib/TypeOfFile"
        );
        assert!(!cls.is_null());
        let static_field_id: jfieldID = match x {
            TypeOfFile::Word => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_WORD,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Word",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Excel => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_EXCEL,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Excel",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::PowerPoint => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_POWERPOINT,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "PowerPoint",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Jpg => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_JPG,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Jpg",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Mkv => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_MKV,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Mkv",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Png => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_PNG,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Png",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Html => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_HTML,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Html",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::MpFour => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_MPFOUR,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "MpFour",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Pdf => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_PDF,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Pdf",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Other => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_OTHER,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Other",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Application => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_APPLICATION,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Application",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Audio => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_AUDIO,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Audio",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::MpThree => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_MPTHREE,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "MpThree",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Gif => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_GIF,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Gif",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Video => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_VIDEO,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Video",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Zip => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_ZIP,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Zip",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Image => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_IMAGE,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Image",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Iso => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_ISO,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Iso",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::ThreeGp => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_THREEGP,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "ThreeGp",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Flv => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_FLV,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Flv",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Document => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_DOCUMENT,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Document",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
            TypeOfFile::Compressed => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_TYPEOFFILE_COMPRESSED,
                    FOREIGN_ENUM_TYPEOFFILE,
                    "Compressed",
                    "Lcom/example/swift_final/lib/TypeOfFile;"
                );
                assert!(!field.is_null());
                field
            }
        };
        assert!(!static_field_id.is_null());
        let ret: jobject =
            unsafe { (**env).GetStaticObjectField.unwrap()(env, cls, static_field_id) };
        assert!(
            !ret.is_null(),
            concat!(
                "Can get value of item in ",
                "com/example/swift_final/lib/TypeOfFile"
            )
        );
        ret
    }
}
impl SwigForeignCLikeEnum for ResponseErrors {
    fn as_jint(&self) -> jint {
        match *self {
            ResponseErrors::ErrorParsingRequest => 0i32,
            ResponseErrors::UnableToDecodeRequest => 1i32,
            ResponseErrors::RedirectedManyTimes => 2i32,
            ResponseErrors::ConnectionTimeout => 3i32,
            ResponseErrors::UnknownError => 4i32,
        }
    }
    fn from_jint(x: jint) -> Self {
        match x {
            0i32 => ResponseErrors::ErrorParsingRequest,
            1i32 => ResponseErrors::UnableToDecodeRequest,
            2i32 => ResponseErrors::RedirectedManyTimes,
            3i32 => ResponseErrors::ConnectionTimeout,
            4i32 => ResponseErrors::UnknownError,
            _ => panic!(
                concat!("{} not expected for ", stringify!(ResponseErrors)),
                x
            ),
        }
    }
}
#[allow(dead_code)]
impl SwigFrom<ResponseErrors> for jobject {
    fn swig_from(x: ResponseErrors, env: *mut JNIEnv) -> jobject {
        let cls: jclass = swig_jni_find_class!(
            FOREIGN_ENUM_RESPONSEERRORS,
            "com/example/swift_final/lib/ResponseErrors"
        );
        assert!(!cls.is_null());
        let static_field_id: jfieldID = match x {
            ResponseErrors::ErrorParsingRequest => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_RESPONSEERRORS_ERRORPARSINGREQUEST,
                    FOREIGN_ENUM_RESPONSEERRORS,
                    "ErrorParsingRequest",
                    "Lcom/example/swift_final/lib/ResponseErrors;"
                );
                assert!(!field.is_null());
                field
            }
            ResponseErrors::UnableToDecodeRequest => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_RESPONSEERRORS_UNABLETODECODEREQUEST,
                    FOREIGN_ENUM_RESPONSEERRORS,
                    "UnableToDecodeRequest",
                    "Lcom/example/swift_final/lib/ResponseErrors;"
                );
                assert!(!field.is_null());
                field
            }
            ResponseErrors::RedirectedManyTimes => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_RESPONSEERRORS_REDIRECTEDMANYTIMES,
                    FOREIGN_ENUM_RESPONSEERRORS,
                    "RedirectedManyTimes",
                    "Lcom/example/swift_final/lib/ResponseErrors;"
                );
                assert!(!field.is_null());
                field
            }
            ResponseErrors::ConnectionTimeout => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_RESPONSEERRORS_CONNECTIONTIMEOUT,
                    FOREIGN_ENUM_RESPONSEERRORS,
                    "ConnectionTimeout",
                    "Lcom/example/swift_final/lib/ResponseErrors;"
                );
                assert!(!field.is_null());
                field
            }
            ResponseErrors::UnknownError => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_RESPONSEERRORS_UNKNOWNERROR,
                    FOREIGN_ENUM_RESPONSEERRORS,
                    "UnknownError",
                    "Lcom/example/swift_final/lib/ResponseErrors;"
                );
                assert!(!field.is_null());
                field
            }
        };
        assert!(!static_field_id.is_null());
        let ret: jobject =
            unsafe { (**env).GetStaticObjectField.unwrap()(env, cls, static_field_id) };
        assert!(
            !ret.is_null(),
            concat!(
                "Can get value of item in ",
                "com/example/swift_final/lib/ResponseErrors"
            )
        );
        ret
    }
}
impl SwigForeignCLikeEnum for FileCategory {
    fn as_jint(&self) -> jint {
        match *self {
            FileCategory::Video => 0i32,
            FileCategory::Document => 1i32,
            FileCategory::Image => 2i32,
            FileCategory::Compressed => 3i32,
            FileCategory::Audio => 4i32,
            FileCategory::Application => 5i32,
            FileCategory::Other => 6i32,
        }
    }
    fn from_jint(x: jint) -> Self {
        match x {
            0i32 => FileCategory::Video,
            1i32 => FileCategory::Document,
            2i32 => FileCategory::Image,
            3i32 => FileCategory::Compressed,
            4i32 => FileCategory::Audio,
            5i32 => FileCategory::Application,
            6i32 => FileCategory::Other,
            _ => panic!(concat!("{} not expected for ", stringify!(FileCategory)), x),
        }
    }
}
#[allow(dead_code)]
impl SwigFrom<FileCategory> for jobject {
    fn swig_from(x: FileCategory, env: *mut JNIEnv) -> jobject {
        let cls: jclass = swig_jni_find_class!(
            FOREIGN_ENUM_FILECATEGORY,
            "com/example/swift_final/lib/FileCategory"
        );
        assert!(!cls.is_null());
        let static_field_id: jfieldID = match x {
            FileCategory::Video => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_FILECATEGORY_VIDEO,
                    FOREIGN_ENUM_FILECATEGORY,
                    "Video",
                    "Lcom/example/swift_final/lib/FileCategory;"
                );
                assert!(!field.is_null());
                field
            }
            FileCategory::Document => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_FILECATEGORY_DOCUMENT,
                    FOREIGN_ENUM_FILECATEGORY,
                    "Document",
                    "Lcom/example/swift_final/lib/FileCategory;"
                );
                assert!(!field.is_null());
                field
            }
            FileCategory::Image => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_FILECATEGORY_IMAGE,
                    FOREIGN_ENUM_FILECATEGORY,
                    "Image",
                    "Lcom/example/swift_final/lib/FileCategory;"
                );
                assert!(!field.is_null());
                field
            }
            FileCategory::Compressed => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_FILECATEGORY_COMPRESSED,
                    FOREIGN_ENUM_FILECATEGORY,
                    "Compressed",
                    "Lcom/example/swift_final/lib/FileCategory;"
                );
                assert!(!field.is_null());
                field
            }
            FileCategory::Audio => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_FILECATEGORY_AUDIO,
                    FOREIGN_ENUM_FILECATEGORY,
                    "Audio",
                    "Lcom/example/swift_final/lib/FileCategory;"
                );
                assert!(!field.is_null());
                field
            }
            FileCategory::Application => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_FILECATEGORY_APPLICATION,
                    FOREIGN_ENUM_FILECATEGORY,
                    "Application",
                    "Lcom/example/swift_final/lib/FileCategory;"
                );
                assert!(!field.is_null());
                field
            }
            FileCategory::Other => {
                let field = swig_jni_get_static_field_id!(
                    FOREIGN_ENUM_FILECATEGORY_OTHER,
                    FOREIGN_ENUM_FILECATEGORY,
                    "Other",
                    "Lcom/example/swift_final/lib/FileCategory;"
                );
                assert!(!field.is_null());
                field
            }
        };
        assert!(!static_field_id.is_null());
        let ret: jobject =
            unsafe { (**env).GetStaticObjectField.unwrap()(env, cls, static_field_id) };
        assert!(
            !ret.is_null(),
            concat!(
                "Can get value of item in ",
                "com/example/swift_final/lib/FileCategory"
            )
        );
        ret
    }
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_Logger_initialiseLogging(
    env: *mut JNIEnv,
    _: jclass,
) -> () {
    let mut ret: () = Logger::initialise_logging();
    ret
}
impl SwigForeignClass for FileType {
    type PointedType = FileType;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_FILETYPE,
            "com/example/swift_final/lib/FileType"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_FILETYPE_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_FILETYPE,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<FileType> = Box::new(this);
        let this: *mut FileType = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut FileType = unsafe { jlong_to_pointer::<FileType>(x).as_mut().unwrap() };
        let x: Box<FileType> = unsafe { Box::from_raw(x) };
        let x: FileType = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut FileType = unsafe { jlong_to_pointer::<FileType>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_FileType_init(
    env: *mut JNIEnv,
    _: jclass,
    extension: jstring,
) -> jlong {
    let mut extension: String = from_jstring_std_string(extension, env);
    let mut extension: &str = extension.as_str();
    let this: FileType = FileType::new(extension);
    let this: Box<FileType> = Box::new(this);
    let this: *mut FileType = Box::into_raw(this);
    this as jlong
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_FileType_do_1getType(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jint {
    let this: &FileType = unsafe { jlong_to_pointer::<FileType>(this).as_mut().unwrap() };
    let mut ret: TypeOfFile = FileType::get_type(this);
    let mut ret: jint = ret.as_jint();
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_FileType_do_1getCategory(
    env: *mut JNIEnv,
    _: jclass,
    type_of_file: jint,
) -> jint {
    let mut type_of_file: TypeOfFile = <TypeOfFile>::from_jint(type_of_file);
    let mut ret: FileCategory = FileType::get_category(type_of_file);
    let mut ret: jint = ret.as_jint();
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_FileType_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut FileType = unsafe { jlong_to_pointer::<FileType>(this).as_mut().unwrap() };
    let this: Box<FileType> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl SwigForeignClass for Authentication {
    type PointedType = Authentication;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_AUTHENTICATION,
            "com/example/swift_final/lib/Authentication"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_AUTHENTICATION_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_AUTHENTICATION,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<Authentication> = Box::new(this);
        let this: *mut Authentication = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut Authentication =
            unsafe { jlong_to_pointer::<Authentication>(x).as_mut().unwrap() };
        let x: Box<Authentication> = unsafe { Box::from_raw(x) };
        let x: Authentication = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut Authentication =
            unsafe { jlong_to_pointer::<Authentication>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_Authentication_init(
    env: *mut JNIEnv,
    _: jclass,
    username: jstring,
    password: internal_aliases::JStringOptStr,
) -> jlong {
    let mut username: String = from_jstring_std_string(username, env);
    let tmp: String;
    let mut password: Option<&str> = if !password.is_null() {
        tmp = from_jstring_std_string(password, env);
        Some(tmp.as_str())
    } else {
        None
    };
    let this: Authentication = Authentication::new(username, password);
    let this: Box<Authentication> = Box::new(this);
    let this: *mut Authentication = Box::into_raw(this);
    this as jlong
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_Authentication_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut Authentication =
        unsafe { jlong_to_pointer::<Authentication>(this).as_mut().unwrap() };
    let this: Box<Authentication> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl SwigForeignClass for DownloadInfo {
    type PointedType = DownloadInfo;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_DOWNLOADINFO,
            "com/example/swift_final/lib/DownloadInfo"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_DOWNLOADINFO_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_DOWNLOADINFO,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<DownloadInfo> = Box::new(this);
        let this: *mut DownloadInfo = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut DownloadInfo = unsafe { jlong_to_pointer::<DownloadInfo>(x).as_mut().unwrap() };
        let x: Box<DownloadInfo> = unsafe { Box::from_raw(x) };
        let x: DownloadInfo = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut DownloadInfo = unsafe { jlong_to_pointer::<DownloadInfo>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_DownloadInfo_init(
    env: *mut JNIEnv,
    _: jclass,
    url: jstring,
    auth: jlong,
) -> jlong {
    let mut url: String = from_jstring_std_string(url, env);
    let mut auth: Option<Authentication> = if auth != 0 {
        let o: Authentication = <Authentication>::unbox_object(auth);
        Some(o)
    } else {
        None
    };
    let this: DownloadInfo = DownloadInfo::constructor(url, auth);
    let this: Box<DownloadInfo> = Box::new(this);
    let this: *mut DownloadInfo = Box::into_raw(this);
    this as jlong
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_DownloadInfo_do_1getUrl(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jstring {
    let this: &DownloadInfo = unsafe { jlong_to_pointer::<DownloadInfo>(this).as_mut().unwrap() };
    let mut ret: &str = DownloadInfo::get_url(this);
    let mut ret: jstring = from_std_str_jstring(ret, env);
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_DownloadInfo_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut DownloadInfo =
        unsafe { jlong_to_pointer::<DownloadInfo>(this).as_mut().unwrap() };
    let this: Box<DownloadInfo> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl SwigForeignClass for RequestInfo {
    type PointedType = RequestInfo;
    fn jni_class() -> jclass {
        swig_jni_find_class!(
            FOREIGN_CLASS_REQUESTINFO,
            "com/example/swift_final/lib/RequestInfo"
        )
    }
    fn jni_class_pointer_field() -> jfieldID {
        swig_jni_get_field_id!(
            FOREIGN_CLASS_REQUESTINFO_MNATIVEOBJ_FIELD,
            FOREIGN_CLASS_REQUESTINFO,
            "mNativeObj",
            "J"
        )
    }
    fn box_object(this: Self) -> jlong {
        let this: Box<RequestInfo> = Box::new(this);
        let this: *mut RequestInfo = Box::into_raw(this);
        this as jlong
    }
    fn unbox_object(x: jlong) -> Self {
        let x: *mut RequestInfo = unsafe { jlong_to_pointer::<RequestInfo>(x).as_mut().unwrap() };
        let x: Box<RequestInfo> = unsafe { Box::from_raw(x) };
        let x: RequestInfo = *x;
        x
    }
    fn to_pointer(x: jlong) -> ::std::ptr::NonNull<Self::PointedType> {
        let x: *mut RequestInfo = unsafe { jlong_to_pointer::<RequestInfo>(x).as_mut().unwrap() };
        ::std::ptr::NonNull::<Self::PointedType>::new(x).unwrap()
    }
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_RequestInfo_init(
    env: *mut JNIEnv,
    _: jclass,
    download_info: jlong,
    filename: jstring,
    file_size: jlong,
    category: jint,
    resumable: jboolean,
) -> jlong {
    let download_info: *mut DownloadInfo = unsafe {
        jlong_to_pointer::<DownloadInfo>(download_info)
            .as_mut()
            .unwrap()
    };
    let download_info: Box<DownloadInfo> = unsafe { Box::from_raw(download_info) };
    let download_info: DownloadInfo = *download_info;
    let mut filename: String = from_jstring_std_string(filename, env);
    let mut file_size: i64 = file_size;
    let mut category: TypeOfFile = <TypeOfFile>::from_jint(category);
    let mut resumable: bool = resumable != 0;
    let this: RequestInfo =
        RequestInfo::new(download_info, filename, file_size, category, resumable);
    let this: Box<RequestInfo> = Box::new(this);
    let this: *mut RequestInfo = Box::into_raw(this);
    this as jlong
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_RequestInfo_do_1getUrl(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jstring {
    let this: &RequestInfo = unsafe { jlong_to_pointer::<RequestInfo>(this).as_mut().unwrap() };
    let mut ret: &str = RequestInfo::get_url(this);
    let mut ret: jstring = from_std_str_jstring(ret, env);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_RequestInfo_do_1getFileSize(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> internal_aliases::JStringOptStr {
    let this: &RequestInfo = unsafe { jlong_to_pointer::<RequestInfo>(this).as_mut().unwrap() };
    let mut ret: Option<String> = RequestInfo::get_file_size(this);
    let mut ret: internal_aliases::JStringOptStr = match ret {
        Some(s) => from_std_str_jstring(&s, env),
        None => ::std::ptr::null_mut(),
    };
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_RequestInfo_do_1getFileSizeInBytes(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jlong {
    let this: &RequestInfo = unsafe { jlong_to_pointer::<RequestInfo>(this).as_mut().unwrap() };
    let mut ret: u64 = RequestInfo::get_file_size_in_bytes(this);
    let mut ret: jlong = <jlong as ::std::convert::TryFrom<u64>>::try_from(ret)
        .expect("invalid u64, in u64 => jlong conversion");
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_RequestInfo_do_1isResumable(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jboolean {
    let this: &RequestInfo = unsafe { jlong_to_pointer::<RequestInfo>(this).as_mut().unwrap() };
    let mut ret: bool = RequestInfo::is_resumable(this);
    let mut ret: jboolean = if ret { 1 as jboolean } else { 0 as jboolean };
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_RequestInfo_do_1typeOfFile(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jint {
    let this: &RequestInfo = unsafe { jlong_to_pointer::<RequestInfo>(this).as_mut().unwrap() };
    let mut ret: TypeOfFile = RequestInfo::type_of_file(this);
    let mut ret: jint = ret.as_jint();
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_RequestInfo_do_1getCategory(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jint {
    let this: &RequestInfo = unsafe { jlong_to_pointer::<RequestInfo>(this).as_mut().unwrap() };
    let mut ret: FileCategory = RequestInfo::get_category(this);
    let mut ret: jint = ret.as_jint();
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_RequestInfo_do_1filename(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jstring {
    let this: &RequestInfo = unsafe { jlong_to_pointer::<RequestInfo>(this).as_mut().unwrap() };
    let mut ret: &str = RequestInfo::filename(this);
    let mut ret: jstring = from_std_str_jstring(ret, env);
    ret
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_RequestInfo_do_1toString(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) -> jstring {
    let this: &RequestInfo = unsafe { jlong_to_pointer::<RequestInfo>(this).as_mut().unwrap() };
    let mut ret: String = RequestInfo::to_string(this);
    let mut ret: jstring = from_std_str_jstring(&ret, env);
    ret
}
#[allow(unused_variables, unused_mut, non_snake_case, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_RequestInfo_do_1delete(
    env: *mut JNIEnv,
    _: jclass,
    this: jlong,
) {
    let this: *mut RequestInfo = unsafe { jlong_to_pointer::<RequestInfo>(this).as_mut().unwrap() };
    let this: Box<RequestInfo> = unsafe { Box::from_raw(this) };
    drop(this);
}
impl DownloadCallback for JavaCallback {
    #[allow(unused_mut)]
    fn response_error(&self, a0: ResponseErrors) {
        swig_assert_eq_size!(::std::os::raw::c_uint, u32);
        swig_assert_eq_size!(::std::os::raw::c_int, i32);
        let env = self.get_jni_env();
        if let Some(env) = env.env {
            let mut a0: jobject = <jobject>::swig_from(a0, env);
            unsafe {
                (**env).CallVoidMethod.unwrap()(env, self.this, self.methods[0usize], a0);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    log::error!(concat!(
                        stringify!(response_error),
                        ": java throw exception"
                    ));
                    (**env).ExceptionDescribe.unwrap()(env);
                    (**env).ExceptionClear.unwrap()(env);
                }
            };
        }
    }
    #[allow(unused_mut)]
    fn status_error(&self, a0: u16, a1: &str) {
        swig_assert_eq_size!(::std::os::raw::c_uint, u32);
        swig_assert_eq_size!(::std::os::raw::c_int, i32);
        let env = self.get_jni_env();
        if let Some(env) = env.env {
            let mut a0: jint = jint::from(a0);
            let mut a1: jstring = from_std_str_jstring(a1, env);
            unsafe {
                (**env).CallVoidMethod.unwrap()(env, self.this, self.methods[1usize], a0, a1);
                if (**env).ExceptionCheck.unwrap()(env) != 0 {
                    log::error!(concat!(stringify!(status_error), ": java throw exception"));
                    (**env).ExceptionDescribe.unwrap()(env);
                    (**env).ExceptionClear.unwrap()(env);
                }
            };
        }
    }
    #[allow(unused_mut)]
    fn is_active(&self) -> bool {
        swig_assert_eq_size!(::std::os::raw::c_uint, u32);
        swig_assert_eq_size!(::std::os::raw::c_int, i32);
        let env = self.get_jni_env();
        let env = env
            .env
            .expect(concat!("Can not get env for ", stringify!(is_active)));
        let mut ret: jboolean;
        unsafe {
            ret = (**env).CallBooleanMethod.unwrap()(env, self.this, self.methods[2usize]);
            if (**env).ExceptionCheck.unwrap()(env) != 0 {
                log::error!(concat!(stringify!(is_active), ": java throw exception"));
                (**env).ExceptionDescribe.unwrap()(env);
                (**env).ExceptionClear.unwrap()(env);
            }
        };
        let mut ret: bool = ret != 0;
        ret
    }
}
#[doc = ""]
impl SwigFrom<jobject> for Box<dyn DownloadCallback> {
    fn swig_from(this: jobject, env: *mut JNIEnv) -> Self {
        let mut cb = JavaCallback::new(this, env);
        cb.methods.reserve(3);
        let class = unsafe { (**env).GetObjectClass.unwrap()(env, cb.this) };
        assert!(
            !class.is_null(),
            "GetObjectClass return null class for DownloadCallback"
        );
        let method_id: jmethodID = unsafe {
            (**env).GetMethodID.unwrap()(
                env,
                class,
                swig_c_str!("responseError"),
                swig_c_str!("(Lcom/example/swift_final/lib/ResponseErrors;)V"),
            )
        };
        assert!(!method_id.is_null(), "Can not find responseError id");
        cb.methods.push(method_id);
        let method_id: jmethodID = unsafe {
            (**env).GetMethodID.unwrap()(
                env,
                class,
                swig_c_str!("statusError"),
                swig_c_str!("(ILjava/lang/String;)V"),
            )
        };
        assert!(!method_id.is_null(), "Can not find statusError id");
        cb.methods.push(method_id);
        let method_id: jmethodID = unsafe {
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("isActive"), swig_c_str!("()Z"))
        };
        assert!(!method_id.is_null(), "Can not find isActive id");
        cb.methods.push(method_id);
        Box::new(cb)
    }
}
#[allow(non_snake_case, unused_variables, unused_mut, unused_unsafe)]
#[no_mangle]
pub extern "C" fn Java_com_example_swift_1final_lib_Downloader_do_1getRequestInfo(
    env: *mut JNIEnv,
    _: jclass,
    download_info: jlong,
    download_callback: jobject,
) -> jlong {
    let download_info: &DownloadInfo = unsafe {
        jlong_to_pointer::<DownloadInfo>(download_info)
            .as_mut()
            .unwrap()
    };
    let mut download_callback: Box<dyn DownloadCallback> =
        <Box<dyn DownloadCallback>>::swig_from(download_callback, env);
    let mut download_callback: &Box<dyn DownloadCallback> = &download_callback;
    let mut ret: Option<RequestInfo> =
        Downloader::get_request_info(download_info, download_callback);
    let mut ret: jlong = match ret {
        Some(x) => {
            let ptr = <RequestInfo>::box_object(x);
            debug_assert_ne!(0, ptr);
            ptr
        }
        None => 0,
    };
    ret
}
static mut JAVA_LANG_EXCEPTION: jclass = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE: jclass = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_WORD: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_EXCEL: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_POWERPOINT: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_JPG: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_MKV: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_PNG: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_HTML: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_MPFOUR: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_PDF: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_OTHER: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_APPLICATION: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_AUDIO: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_MPTHREE: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_GIF: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_VIDEO: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_ZIP: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_IMAGE: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_ISO: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_THREEGP: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_FLV: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_DOCUMENT: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_TYPEOFFILE_COMPRESSED: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_AUTHENTICATION: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_AUTHENTICATION_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_DOUBLE: jclass = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_DOUBLE_OF: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_DOUBLE_EMPTY: jmethodID = ::std::ptr::null_mut();
static mut JAVA_LANG_STRING: jclass = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_LONG: jclass = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_LONG_OF: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_LONG_EMPTY: jmethodID = ::std::ptr::null_mut();
static mut JAVA_LANG_BYTE: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_BYTE_BYTE_VALUE: jmethodID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_FILETYPE: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_FILETYPE_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_REQUESTINFO: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_REQUESTINFO_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut JAVA_LANG_FLOAT: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_FLOAT_FLOAT_VALUE: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_INT: jclass = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_INT_OF: jmethodID = ::std::ptr::null_mut();
static mut JAVA_UTIL_OPTIONAL_INT_EMPTY: jmethodID = ::std::ptr::null_mut();
static mut JAVA_LANG_INTEGER: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_INTEGER_INT_VALUE: jmethodID = ::std::ptr::null_mut();
static mut JAVA_LANG_LONG: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_LONG_LONG_VALUE: jmethodID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_RESPONSEERRORS: jclass = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_RESPONSEERRORS_ERRORPARSINGREQUEST: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_RESPONSEERRORS_UNABLETODECODEREQUEST: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_RESPONSEERRORS_REDIRECTEDMANYTIMES: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_RESPONSEERRORS_CONNECTIONTIMEOUT: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_RESPONSEERRORS_UNKNOWNERROR: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_FILECATEGORY: jclass = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_FILECATEGORY_VIDEO: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_FILECATEGORY_DOCUMENT: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_FILECATEGORY_IMAGE: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_FILECATEGORY_COMPRESSED: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_FILECATEGORY_AUDIO: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_FILECATEGORY_APPLICATION: jfieldID = ::std::ptr::null_mut();
static mut FOREIGN_ENUM_FILECATEGORY_OTHER: jfieldID = ::std::ptr::null_mut();
static mut JAVA_LANG_DOUBLE: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_DOUBLE_DOUBLE_VALUE_METHOD: jmethodID = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_DOWNLOADINFO: jclass = ::std::ptr::null_mut();
static mut FOREIGN_CLASS_DOWNLOADINFO_MNATIVEOBJ_FIELD: jfieldID = ::std::ptr::null_mut();
static mut JAVA_LANG_SHORT: jclass = ::std::ptr::null_mut();
static mut JAVA_LANG_SHORT_SHORT_VALUE: jmethodID = ::std::ptr::null_mut();
#[no_mangle]
pub extern "system" fn JNI_OnLoad(
    java_vm: *mut JavaVM,
    _reserved: *mut ::std::os::raw::c_void,
) -> jint {
    log::debug!("JNI_OnLoad begin");
    assert!(!java_vm.is_null());
    let mut env: *mut JNIEnv = ::std::ptr::null_mut();
    let res = unsafe {
        (**java_vm).GetEnv.unwrap()(
            java_vm,
            (&mut env) as *mut *mut JNIEnv as *mut *mut ::std::os::raw::c_void,
            SWIG_JNI_VERSION,
        )
    };
    if res != (JNI_OK as jint) {
        panic!("JNI GetEnv in JNI_OnLoad failed, return code {}", res);
    }
    assert!(!env.is_null());
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Exception"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Exception")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Exception")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_EXCEPTION = class;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("com/example/swift_final/lib/TypeOfFile"));
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "com/example/swift_final/lib/TypeOfFile"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "com/example/swift_final/lib/TypeOfFile"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_ENUM_TYPEOFFILE = class;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Word"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Word",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_WORD = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Excel"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Excel",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_EXCEL = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("PowerPoint"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "PowerPoint",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_POWERPOINT = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Jpg"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Jpg",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_JPG = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Mkv"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Mkv",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_MKV = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Png"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Png",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_PNG = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Html"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Html",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_HTML = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("MpFour"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "MpFour",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_MPFOUR = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Pdf"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Pdf",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_PDF = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Other"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Other",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_OTHER = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Application"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Application",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_APPLICATION = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Audio"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Audio",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_AUDIO = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("MpThree"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "MpThree",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_MPTHREE = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Gif"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Gif",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_GIF = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Video"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Video",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_VIDEO = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Zip"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Zip",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_ZIP = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Image"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Image",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_IMAGE = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Iso"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Iso",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_ISO = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("ThreeGp"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "ThreeGp",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_THREEGP = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Flv"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Flv",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_FLV = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Document"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Document",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_DOCUMENT = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Compressed"),
            swig_c_str!("Lcom/example/swift_final/lib/TypeOfFile;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/TypeOfFile",
                " method ",
                "Compressed",
                " sig ",
                "Lcom/example/swift_final/lib/TypeOfFile;",
                " failed"
            )
        );
        FOREIGN_ENUM_TYPEOFFILE_COMPRESSED = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("com/example/swift_final/lib/Authentication"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "com/example/swift_final/lib/Authentication"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "com/example/swift_final/lib/Authentication"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_AUTHENTICATION = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/Authentication",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_AUTHENTICATION_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("java/util/OptionalDouble"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalDouble")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalDouble")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_UTIL_OPTIONAL_DOUBLE = class;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("of"),
            swig_c_str!("(D)Ljava/util/OptionalDouble;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalDouble",
                " method ",
                "of",
                " sig ",
                "(D)Ljava/util/OptionalDouble;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_DOUBLE_OF = method_id;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("empty"),
            swig_c_str!("()Ljava/util/OptionalDouble;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalDouble",
                " method ",
                "empty",
                " sig ",
                "()Ljava/util/OptionalDouble;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_DOUBLE_EMPTY = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/String"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/String")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/String")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_STRING = class;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("java/util/OptionalLong"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalLong")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalLong")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_UTIL_OPTIONAL_LONG = class;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("of"),
            swig_c_str!("(J)Ljava/util/OptionalLong;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalLong",
                " method ",
                "of",
                " sig ",
                "(J)Ljava/util/OptionalLong;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_LONG_OF = method_id;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("empty"),
            swig_c_str!("()Ljava/util/OptionalLong;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalLong",
                " method ",
                "empty",
                " sig ",
                "()Ljava/util/OptionalLong;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_LONG_EMPTY = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Byte"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Byte")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Byte")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_BYTE = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("byteValue"), swig_c_str!("()B"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Byte",
                " method ",
                "byteValue",
                " sig ",
                "()B",
                " failed"
            )
        );
        JAVA_LANG_BYTE_BYTE_VALUE = method_id;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("com/example/swift_final/lib/FileType"));
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "com/example/swift_final/lib/FileType"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "com/example/swift_final/lib/FileType"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_FILETYPE = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/FileType",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_FILETYPE_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref =
            (**env).FindClass.unwrap()(env, swig_c_str!("com/example/swift_final/lib/RequestInfo"));
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "com/example/swift_final/lib/RequestInfo"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "com/example/swift_final/lib/RequestInfo"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_REQUESTINFO = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/RequestInfo",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_REQUESTINFO_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Float"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Float")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Float")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_FLOAT = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("floatValue"), swig_c_str!("()F"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Float",
                " method ",
                "floatValue",
                " sig ",
                "()F",
                " failed"
            )
        );
        JAVA_LANG_FLOAT_FLOAT_VALUE = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/util/OptionalInt"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalInt")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/util/OptionalInt")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_UTIL_OPTIONAL_INT = class;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("of"),
            swig_c_str!("(I)Ljava/util/OptionalInt;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalInt",
                " method ",
                "of",
                " sig ",
                "(I)Ljava/util/OptionalInt;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_INT_OF = method_id;
        let method_id: jmethodID = (**env).GetStaticMethodID.unwrap()(
            env,
            class,
            swig_c_str!("empty"),
            swig_c_str!("()Ljava/util/OptionalInt;"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetStaticMethodID for class ",
                "java/util/OptionalInt",
                " method ",
                "empty",
                " sig ",
                "()Ljava/util/OptionalInt;",
                " failed"
            )
        );
        JAVA_UTIL_OPTIONAL_INT_EMPTY = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Integer"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Integer")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Integer")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_INTEGER = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("intValue"), swig_c_str!("()I"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Integer",
                " method ",
                "intValue",
                " sig ",
                "()I",
                " failed"
            )
        );
        JAVA_LANG_INTEGER_INT_VALUE = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Long"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Long")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Long")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_LONG = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("longValue"), swig_c_str!("()J"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Long",
                " method ",
                "longValue",
                " sig ",
                "()J",
                " failed"
            )
        );
        JAVA_LANG_LONG_LONG_VALUE = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("com/example/swift_final/lib/ResponseErrors"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "com/example/swift_final/lib/ResponseErrors"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "com/example/swift_final/lib/ResponseErrors"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_ENUM_RESPONSEERRORS = class;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("ErrorParsingRequest"),
            swig_c_str!("Lcom/example/swift_final/lib/ResponseErrors;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/ResponseErrors",
                " method ",
                "ErrorParsingRequest",
                " sig ",
                "Lcom/example/swift_final/lib/ResponseErrors;",
                " failed"
            )
        );
        FOREIGN_ENUM_RESPONSEERRORS_ERRORPARSINGREQUEST = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("UnableToDecodeRequest"),
            swig_c_str!("Lcom/example/swift_final/lib/ResponseErrors;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/ResponseErrors",
                " method ",
                "UnableToDecodeRequest",
                " sig ",
                "Lcom/example/swift_final/lib/ResponseErrors;",
                " failed"
            )
        );
        FOREIGN_ENUM_RESPONSEERRORS_UNABLETODECODEREQUEST = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("RedirectedManyTimes"),
            swig_c_str!("Lcom/example/swift_final/lib/ResponseErrors;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/ResponseErrors",
                " method ",
                "RedirectedManyTimes",
                " sig ",
                "Lcom/example/swift_final/lib/ResponseErrors;",
                " failed"
            )
        );
        FOREIGN_ENUM_RESPONSEERRORS_REDIRECTEDMANYTIMES = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("ConnectionTimeout"),
            swig_c_str!("Lcom/example/swift_final/lib/ResponseErrors;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/ResponseErrors",
                " method ",
                "ConnectionTimeout",
                " sig ",
                "Lcom/example/swift_final/lib/ResponseErrors;",
                " failed"
            )
        );
        FOREIGN_ENUM_RESPONSEERRORS_CONNECTIONTIMEOUT = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("UnknownError"),
            swig_c_str!("Lcom/example/swift_final/lib/ResponseErrors;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/ResponseErrors",
                " method ",
                "UnknownError",
                " sig ",
                "Lcom/example/swift_final/lib/ResponseErrors;",
                " failed"
            )
        );
        FOREIGN_ENUM_RESPONSEERRORS_UNKNOWNERROR = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("com/example/swift_final/lib/FileCategory"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "com/example/swift_final/lib/FileCategory"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "com/example/swift_final/lib/FileCategory"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_ENUM_FILECATEGORY = class;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Video"),
            swig_c_str!("Lcom/example/swift_final/lib/FileCategory;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/FileCategory",
                " method ",
                "Video",
                " sig ",
                "Lcom/example/swift_final/lib/FileCategory;",
                " failed"
            )
        );
        FOREIGN_ENUM_FILECATEGORY_VIDEO = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Document"),
            swig_c_str!("Lcom/example/swift_final/lib/FileCategory;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/FileCategory",
                " method ",
                "Document",
                " sig ",
                "Lcom/example/swift_final/lib/FileCategory;",
                " failed"
            )
        );
        FOREIGN_ENUM_FILECATEGORY_DOCUMENT = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Image"),
            swig_c_str!("Lcom/example/swift_final/lib/FileCategory;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/FileCategory",
                " method ",
                "Image",
                " sig ",
                "Lcom/example/swift_final/lib/FileCategory;",
                " failed"
            )
        );
        FOREIGN_ENUM_FILECATEGORY_IMAGE = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Compressed"),
            swig_c_str!("Lcom/example/swift_final/lib/FileCategory;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/FileCategory",
                " method ",
                "Compressed",
                " sig ",
                "Lcom/example/swift_final/lib/FileCategory;",
                " failed"
            )
        );
        FOREIGN_ENUM_FILECATEGORY_COMPRESSED = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Audio"),
            swig_c_str!("Lcom/example/swift_final/lib/FileCategory;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/FileCategory",
                " method ",
                "Audio",
                " sig ",
                "Lcom/example/swift_final/lib/FileCategory;",
                " failed"
            )
        );
        FOREIGN_ENUM_FILECATEGORY_AUDIO = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Application"),
            swig_c_str!("Lcom/example/swift_final/lib/FileCategory;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/FileCategory",
                " method ",
                "Application",
                " sig ",
                "Lcom/example/swift_final/lib/FileCategory;",
                " failed"
            )
        );
        FOREIGN_ENUM_FILECATEGORY_APPLICATION = field_id;
        let field_id: jfieldID = (**env).GetStaticFieldID.unwrap()(
            env,
            class,
            swig_c_str!("Other"),
            swig_c_str!("Lcom/example/swift_final/lib/FileCategory;"),
        );
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/FileCategory",
                " method ",
                "Other",
                " sig ",
                "Lcom/example/swift_final/lib/FileCategory;",
                " failed"
            )
        );
        FOREIGN_ENUM_FILECATEGORY_OTHER = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Double"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Double")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Double")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_DOUBLE = class;
        let method_id: jmethodID = (**env).GetMethodID.unwrap()(
            env,
            class,
            swig_c_str!("doubleValue"),
            swig_c_str!("()D"),
        );
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Double",
                " method ",
                "doubleValue",
                " sig ",
                "()D",
                " failed"
            )
        );
        JAVA_LANG_DOUBLE_DOUBLE_VALUE_METHOD = method_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(
            env,
            swig_c_str!("com/example/swift_final/lib/DownloadInfo"),
        );
        assert!(
            !class_local_ref.is_null(),
            concat!(
                "FindClass failed for ",
                "com/example/swift_final/lib/DownloadInfo"
            )
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!(
                "FindClass failed for ",
                "com/example/swift_final/lib/DownloadInfo"
            )
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        FOREIGN_CLASS_DOWNLOADINFO = class;
        let field_id: jfieldID =
            (**env).GetFieldID.unwrap()(env, class, swig_c_str!("mNativeObj"), swig_c_str!("J"));
        assert!(
            !field_id.is_null(),
            concat!(
                "GetStaticFieldID for class ",
                "com/example/swift_final/lib/DownloadInfo",
                " method ",
                "mNativeObj",
                " sig ",
                "J",
                " failed"
            )
        );
        FOREIGN_CLASS_DOWNLOADINFO_MNATIVEOBJ_FIELD = field_id;
    }
    unsafe {
        let class_local_ref = (**env).FindClass.unwrap()(env, swig_c_str!("java/lang/Short"));
        assert!(
            !class_local_ref.is_null(),
            concat!("FindClass failed for ", "java/lang/Short")
        );
        let class = (**env).NewGlobalRef.unwrap()(env, class_local_ref);
        assert!(
            !class.is_null(),
            concat!("FindClass failed for ", "java/lang/Short")
        );
        (**env).DeleteLocalRef.unwrap()(env, class_local_ref);
        JAVA_LANG_SHORT = class;
        let method_id: jmethodID =
            (**env).GetMethodID.unwrap()(env, class, swig_c_str!("shortValue"), swig_c_str!("()S"));
        assert!(
            !method_id.is_null(),
            concat!(
                "GetMethodID for class ",
                "java/lang/Short",
                " method ",
                "shortValue",
                " sig ",
                "()S",
                " failed"
            )
        );
        JAVA_LANG_SHORT_SHORT_VALUE = method_id;
    }
    SWIG_JNI_VERSION
}
#[no_mangle]
pub extern "system" fn JNI_OnUnload(java_vm: *mut JavaVM, _reserved: *mut ::std::os::raw::c_void) {
    log::debug!("JNI_OnUnLoad begin");
    assert!(!java_vm.is_null());
    let mut env: *mut JNIEnv = ::std::ptr::null_mut();
    let res = unsafe {
        (**java_vm).GetEnv.unwrap()(
            java_vm,
            (&mut env) as *mut *mut JNIEnv as *mut *mut ::std::os::raw::c_void,
            SWIG_JNI_VERSION,
        )
    };
    if res != (JNI_OK as jint) {
        panic!("JNI GetEnv in JNI_OnLoad failed, return code {}", res);
    }
    assert!(!env.is_null());
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_EXCEPTION);
        JAVA_LANG_EXCEPTION = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_ENUM_TYPEOFFILE);
        FOREIGN_ENUM_TYPEOFFILE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_AUTHENTICATION);
        FOREIGN_CLASS_AUTHENTICATION = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_UTIL_OPTIONAL_DOUBLE);
        JAVA_UTIL_OPTIONAL_DOUBLE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_STRING);
        JAVA_LANG_STRING = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_UTIL_OPTIONAL_LONG);
        JAVA_UTIL_OPTIONAL_LONG = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_BYTE);
        JAVA_LANG_BYTE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_FILETYPE);
        FOREIGN_CLASS_FILETYPE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_REQUESTINFO);
        FOREIGN_CLASS_REQUESTINFO = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_FLOAT);
        JAVA_LANG_FLOAT = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_UTIL_OPTIONAL_INT);
        JAVA_UTIL_OPTIONAL_INT = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_INTEGER);
        JAVA_LANG_INTEGER = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_LONG);
        JAVA_LANG_LONG = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_ENUM_RESPONSEERRORS);
        FOREIGN_ENUM_RESPONSEERRORS = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_ENUM_FILECATEGORY);
        FOREIGN_ENUM_FILECATEGORY = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_DOUBLE);
        JAVA_LANG_DOUBLE = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, FOREIGN_CLASS_DOWNLOADINFO);
        FOREIGN_CLASS_DOWNLOADINFO = ::std::ptr::null_mut()
    }
    unsafe {
        (**env).DeleteGlobalRef.unwrap()(env, JAVA_LANG_SHORT);
        JAVA_LANG_SHORT = ::std::ptr::null_mut()
    }
}
