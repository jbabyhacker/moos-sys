#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fmt::Debug;
use std::os::raw::c_void;
use std::slice;

/// Type to populate when sending data to the MOOSDB as well as populated when
/// mail is received from the MOOSDB.
#[derive(Debug, Clone)]
pub enum MoosMessageData {
    DOUBLE(f64),
    STRING(&'static str),
}

/// Callbacks that are called by MOOS. The implementer must have `*mut MoosApp` as a member
/// of its struct.
pub trait MoosInterface {
    extern "C" fn iterate(app: *mut c_void) -> bool;
    extern "C" fn on_start_up(app: *mut c_void) -> bool;
    extern "C" fn on_connect_to_server(app: *mut c_void) -> bool;
    /// Called when new mail is received from the MOOSDB
    fn on_new_mail(app: *mut c_void, d: HashMap<String, MoosMessageData>) -> bool;

    /// Implementer of `MoosInterface` is required to provide a pointer to `MoosApp`
    /// in the implementer's struct.
    fn app(&mut self) -> &'static mut MoosApp;

    /// Called when new mail is received from the MOOSDB. It is then repackaged
    /// into a Rust type and passed into `on_new_mail`.
    extern "C" fn on_new_raw_mail(app: *mut c_void, mail: *const Envelope, size: u32) -> bool {
        let envelopes: &[Envelope] =
            unsafe { slice::from_raw_parts(mail as *const Envelope, size as usize) };

        let mut d = HashMap::new();

        for envelope in envelopes {
            d.insert(
                unsafe { CStr::from_ptr(envelope.name) }
                    .to_str()
                    .unwrap()
                    .to_string(),
                match envelope.kind {
                    DataType_DOUBLE => MoosMessageData::DOUBLE(envelope.d_value),
                    DataType_STRING => MoosMessageData::STRING(
                        unsafe { CStr::from_ptr(envelope.s_value) }
                            .to_str()
                            .unwrap(),
                    ),
                    _ => unreachable!(),
                },
            );
        }

        Self::on_new_mail(app, d)
    }

    /// Runs the app with the given `name` and `mission` file.
    fn run(&mut self, name: &str, mission: &str)
    where
        Self: std::marker::Sized,
    {
        let app: &mut MoosApp = self.app();
        app.set_target(self);
        app.run(name, mission);
    }
}

impl MoosApp {
    /// Allocates a new MoosApp.
    pub fn new<I: MoosInterface>() -> *mut Self {
        let app: *mut MoosApp = unsafe { newMoosApp() };

        unsafe {
            MoosApp_setIterateCallback(app, Some(I::iterate));
            MoosApp_setOnStartUpCallback(app, Some(I::on_start_up));
            MoosApp_setOnConnectToServerCallback(app, Some(I::on_connect_to_server));
            MoosApp_setOnNewMailCallback(app, Some(I::on_new_raw_mail));
        }

        app
    }

    /// Runs the MoosApp.
    fn run(&mut self, name: &str, mission: &str) -> bool {
        let c_name = CString::new(name).unwrap();
        let c_mission = CString::new(mission).unwrap();

        unsafe { MoosApp_run(self, c_name.as_ptr(), c_mission.as_ptr()) }
    }

    /// Sets a pointer to the target struct which is passed in through
    /// the `MoosInterface` callbacks. This is so the implementer of
    /// `MoosInterface` can have access to its own data and functions.
    fn set_target<T>(&mut self, target: &mut T) {
        let state_ptr: *mut c_void = target as *mut _ as *mut c_void;
        unsafe { MoosApp_setTarget(self, state_ptr) }
    }

    /// Inserts a variable into the MOOSDB.
    pub fn notify(&mut self, name: &str, value: &MoosMessageData) -> bool {
        let c_name = CString::new(name).unwrap();

        match value {
            MoosMessageData::DOUBLE(as_f64) => unsafe {
                MoosApp_notifyDouble(self, c_name.as_ptr(), *as_f64)
            },
            MoosMessageData::STRING(as_string) => {
                let c_value = CString::new(as_string.clone()).unwrap();
                unsafe { MoosApp_notifyString(self, c_name.as_ptr(), c_value.as_ptr()) }
            }
        }
    }

    /// Registers a variable to receive callbacks on when updated in the MOOSDB.
    pub fn register(&mut self, name: &str, interval: f64) -> bool {
        let c_name = CString::new(name).unwrap();

        unsafe { MoosApp_register(self, c_name.as_ptr(), interval) }
    }
}

fn get_app<FromType, ToType>(app: *mut FromType) -> &'static mut ToType {
    unsafe { &mut *(app as *mut ToType) }
}

/// Obtains the struct that implements `MoosInterface`.
pub fn this<ToType>(app: *mut c_void) -> &'static mut ToType {
    get_app::<c_void, ToType>(app)
}

/// Obtains `MoosApp` from a `MoosApp` member.
pub fn to_app(app: *mut MoosApp) -> &'static mut MoosApp {
    get_app::<MoosApp, MoosApp>(app)
}

impl Drop for MoosApp {
    fn drop(&mut self) {
        unsafe { deleteMoosApp(self) }
    }
}
