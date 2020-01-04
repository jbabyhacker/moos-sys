use moos_sys::MoosInterface;
use std::collections::HashMap;
use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_void};

// Create your struct and include MoosApp as a member.
pub struct DemoMoosApp {
    base_app: *mut moos_sys::MoosApp,
    value: i32,
    data: HashMap<String, moos_sys::MoosMessageData>,
}

// Initialize your struct
impl DemoMoosApp {
    pub fn new() -> Self {
        DemoMoosApp {
            base_app: moos_sys::MoosApp::new::<DemoMoosApp>(),
            value: 0,
            data: Default::default(),
        }
    }

    pub fn do_work(&mut self) {
        println!("doing work");
    }
}

// Implement the MoosInterface for your struct, these are the callbacks
// that are to be called.
impl MoosInterface for DemoMoosApp {
    extern "C" fn iterate(app_ptr: *mut c_void) -> bool {
        let this_app = moos_sys::this::<DemoMoosApp>(app_ptr);

        this_app.value += 1;
        println!("Value: {}", this_app.value);

        this_app.do_work();
        let base_app: &mut moos_sys::MoosApp = this_app.base_app();

        base_app.notify(
            "X",
            &moos_sys::MoosMessageData::DOUBLE(this_app.value as f64),
        );
        base_app.notify("Y", &moos_sys::MoosMessageData::STRING("test"));

        true
    }

    extern "C" fn on_start_up(app_ptr: *mut c_void) -> bool {
        println!("onStartUp");
        let this_app = moos_sys::this::<DemoMoosApp>(app_ptr);
        let base_app: &mut moos_sys::MoosApp = this_app.base_app();

        let food: Option<f64> = base_app.app_param("Food");
        let taste: Option<&str> = base_app.app_param("Taste");
        let example: Option<f64> = base_app.global_param("ExampleParam");
        let str_param: Option<&str> = base_app.global_param("CoolParam");

        println!("Food: {:?}", food);
        println!("Taste: {:?}", taste);
        println!("ExampleParam: {:?}", example);
        println!("CoolParam: {:?}", str_param);

        true
    }

    extern "C" fn on_connect_to_server(app_ptr: *mut c_void) -> bool {
        println!("onConnectToServer");
        let this_app = moos_sys::this::<DemoMoosApp>(app_ptr);
        let base_app: &mut moos_sys::MoosApp = this_app.base_app();
        base_app.register("X", 0.0);
        base_app.register("Y", 0.0);
        true
    }

    fn on_new_mail(app_ptr: *mut c_void, data: HashMap<String, moos_sys::MoosMessageData>) -> bool {
        let this_app = moos_sys::this::<DemoMoosApp>(app_ptr);
        this_app.data = data;

        println!("setMail - {:?}", this_app.data);

        true
    }

    fn base_app(&mut self) -> &'static mut moos_sys::MoosApp {
        moos_sys::to_app(self.base_app)
    }

    extern "C" fn on_build_report(app_ptr: *mut c_void) -> *const c_char {
        let this_app = moos_sys::this::<DemoMoosApp>(app_ptr);
        let report = CString::new(format!("My report {}", this_app.value)).unwrap();
        let c_report = report.as_ptr();
        mem::forget(report);

        c_report
    }
}

fn main() {
    let mut moos_app = DemoMoosApp::new();
    let mut root = crate_root::root().unwrap();
    root.push("examples");
    root.push("attack.moos");
    moos_app.run("eat", &root);
}
