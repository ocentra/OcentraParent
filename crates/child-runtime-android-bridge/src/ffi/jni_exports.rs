use jni::{
    objects::{JClass, JString},
    sys::{jboolean, jint, jlong, jstring},
    JNIEnv,
};

use super::{bridge_health, bridge_lifecycle};

fn java_string(env: &JNIEnv<'_>, value: &str) -> jstring {
    env.new_string(value)
        .map(JString::into_raw)
        .unwrap_or(std::ptr::null_mut())
}

// SAFETY: this cdylib is the sole owner of the ChildAgentComposition nativeStart JNI symbol.
#[unsafe(no_mangle)]
pub extern "system" fn Java_ca_ocentra_child_agent_ChildAgentComposition_nativeStart(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    durable_root: JString<'_>,
) -> jlong {
    let root = match env.get_string(&durable_root) {
        Ok(value) => value.to_string_lossy().into_owned(),
        Err(error) => {
            bridge_lifecycle::record_error(format!(
                "native child-runtime bridge durable root is invalid: {error}"
            ));
            return 0;
        }
    };
    bridge_lifecycle::start(root)
}

// SAFETY: this cdylib is the sole owner of the ChildAgentComposition nativeReadiness JNI symbol.
#[unsafe(no_mangle)]
pub extern "system" fn Java_ca_ocentra_child_agent_ChildAgentComposition_nativeReadiness(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) -> jint {
    bridge_health::readiness(handle)
}

// SAFETY: this cdylib is the sole owner of the ChildAgentComposition nativeDomainFlowCount JNI symbol.
#[unsafe(no_mangle)]
pub extern "system" fn Java_ca_ocentra_child_agent_ChildAgentComposition_nativeDomainFlowCount(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) -> jint {
    bridge_health::domain_flow_count(handle)
}

// SAFETY: this cdylib is the sole owner of the ChildAgentComposition nativeLastError JNI symbol.
#[unsafe(no_mangle)]
pub extern "system" fn Java_ca_ocentra_child_agent_ChildAgentComposition_nativeLastError(
    env: JNIEnv<'_>,
    _class: JClass<'_>,
) -> jstring {
    java_string(&env, &bridge_lifecycle::last_error())
}

// SAFETY: this cdylib is the sole owner of the ChildAgentComposition nativeStop JNI symbol.
#[unsafe(no_mangle)]
pub extern "system" fn Java_ca_ocentra_child_agent_ChildAgentComposition_nativeStop(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jlong,
) -> jboolean {
    jboolean::from(bridge_lifecycle::stop(handle))
}
