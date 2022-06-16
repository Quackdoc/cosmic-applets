mod imp;

use crate::{workspace_object::WorkspaceObject, Activate, TX};
use glib::Object;
use gtk4::{glib, prelude::*, subclass::prelude::*, ToggleButton};

glib::wrapper! {
    pub struct WorkspaceButton(ObjectSubclass<imp::WorkspaceButton>)
        @extends gtk4::Box, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Actionable, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl WorkspaceButton {
    pub fn new() -> Self {
        let self_ = Object::new(&[]).expect("Failed to create `WorkspaceButton`.");
        let imp = imp::WorkspaceButton::from_instance(&self_);

        let tb = ToggleButton::with_label("");
        self_.append(&tb);

        imp.button.replace(tb);

        self_
    }

    pub fn set_workspace_object(&self, obj: &WorkspaceObject) {
        let imp = imp::WorkspaceButton::from_instance(&self);
        let old_button = imp.button.take();
        self.remove(&old_button);

        let id = obj.id();
        let new_button = ToggleButton::with_label(&id);
        new_button.set_active(obj.active() == 0);
        if obj.active() == 1 {
            new_button.add_css_class("alert");
        }
        self.append(&new_button);
        new_button.connect_clicked(move |_| {
            let id_clone = id.clone();
            glib::MainContext::default().spawn_local(async move {
                TX.get().unwrap().send(id_clone).await.unwrap();
            });
        });

        imp.button.replace(new_button);
    }
}
