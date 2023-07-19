/* window.rs
 *
 * Copyright 2023 Zoey
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/stories/window.ui")]
    pub struct GnomestoriesWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GnomestoriesWindow {
        const NAME: &'static str = "GnomestoriesWindow";
        type Type = super::GnomestoriesWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GnomestoriesWindow {}
    impl WidgetImpl for GnomestoriesWindow {}
    impl WindowImpl for GnomestoriesWindow {}
    impl ApplicationWindowImpl for GnomestoriesWindow {}
    impl AdwApplicationWindowImpl for GnomestoriesWindow {}
}

glib::wrapper! {
    pub struct GnomestoriesWindow(ObjectSubclass<imp::GnomestoriesWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl GnomestoriesWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
