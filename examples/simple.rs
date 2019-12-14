use moos_sys::MoosInterface;
use std::collections::HashMap;
use std::os::raw::c_void;

// Create your struct and include MoosApp as a member.
pub struct DemoMoosApp {
    app: *mut moos_sys::MoosApp,
    value: i32,
    data: HashMap<String, moos_sys::MoosMessageData>,
}

// Initialize your struct
impl DemoMoosApp {
    pub fn new() -> Self {
        DemoMoosApp {
            app: moos_sys::MoosApp::new::<DemoMoosApp>(),
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
    extern "C" fn iterate(app: *mut c_void) -> bool {
        let this_app = moos_sys::this::<DemoMoosApp>(app);

        println!("Value: {}", this_app.value);

        this_app.do_work();
        let a: &mut moos_sys::MoosApp = this_app.app();

        a.notify(
            "X",
            &moos_sys::MoosMessageData::DOUBLE(this_app.value as f64),
        );
        a.notify("Y", &moos_sys::MoosMessageData::STRING("test"));

        true
    }

    extern "C" fn on_start_up(app: *mut c_void) -> bool {
        println!("onStartUp");
        let this_app = moos_sys::this::<DemoMoosApp>(app);
        let a: &mut moos_sys::MoosApp = this_app.app();
        let mut food = moos_sys::MoosMessageData::DOUBLE(0.0);
        let mut taste = moos_sys::MoosMessageData::STRING("");
        let mut example = moos_sys::MoosMessageData::DOUBLE(0.0);

        a.app_param("Food", &mut food);
        a.app_param("Taste", &mut taste);
        a.global_param("ExampleParam", &mut example);
        println!("Food: {:?}", food);
        println!("Taste: {:?}", taste);
        println!("Example: {:?}", example);

        true
    }

    extern "C" fn on_connect_to_server(app: *mut c_void) -> bool {
        println!("onConnectToServer");
        let this_app = moos_sys::this::<DemoMoosApp>(app);
        let d: &mut moos_sys::MoosApp = this_app.app();
        d.register("X", 0.0);
        d.register("Y", 0.0);
        true
    }

    fn on_new_mail(app: *mut c_void, d: HashMap<String, moos_sys::MoosMessageData>) -> bool {
        let this_app = moos_sys::this::<DemoMoosApp>(app);
        this_app.data = d;

        println!("setMail - {:?}", this_app.data);
        this_app.value += 1;
        true
    }

    fn app(&mut self) -> &'static mut moos_sys::MoosApp {
        moos_sys::to_app(self.app)
    }
}

fn main() {
    let mut moos_app = DemoMoosApp::new();
    let mut root = crate_root::root().unwrap();
    root.push("examples");
    root.push("attack.moos");
    moos_app.run("eat", &root);
}
