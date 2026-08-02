pub fn init_uiproxy_object_fix(env: &mut crate::environment::Environment) {
    let superclass = env.objc_root_env.get_class("NSObject")
        .expect("Error: NSObject root not found.");
        
    let mut cls = env.objc_root_env.new_class("UIProxyObject", superclass);

    cls.add_method(
        sel!(initWithCoder:),
        ui_proxy_object_init_with_coder as extern "C" fn(crate::objc::Id, crate::objc::Id, crate::objc::Id) -> crate::objc::Id,
    );

    cls.add_method(
        sel!(targetForAction:withSender:),
        ui_proxy_object_target_for_action as extern "C" fn(crate::objc::Id, crate::objc::Id, crate::objc::Id, crate::objc::Id) -> crate::objc::Id,
    );

    env.objc_root_env.register_class(cls);
}

extern "C" fn ui_proxy_object_init_with_coder(this: crate::objc::Id, _cmd: crate::objc::Id, _coder: crate::objc::Id) -> crate::objc::Id {
    this
}

extern "C" fn ui_proxy_object_target_for_action(_this: crate::objc::Id, _cmd: crate::objc::Id, _action: crate::objc::Id, _sender: crate::objc::Id) -> crate::objc::Id {
    0 
}
