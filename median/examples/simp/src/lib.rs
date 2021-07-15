use median::{
    attr::{AttrBuilder, AttrType},
    builder::MaxWrappedBuilder,
    class::Class,
    clock::ClockHandle,
    inlet::MaxInlet,
    num::{Float64, Int64},
    object::MaxObj,
    outlet::OutList,
    post,
    symbol::SymbolRef,
    wrapper::{
        attr_get_tramp, attr_set_tramp, tramp, MaxObjWrapped, MaxObjWrapper, WrapperWrapped,
    },
};

use std::convert::{From, TryFrom};

median::external! {
    //#[name="simp"]
    pub struct Simp {
        value: Int64,
        fvalue: Float64,
        _v: String,
        clock: ClockHandle,
        list_out: OutList,
    }

    impl MaxObjWrapped<Simp> for Simp {
        fn new(builder: &mut dyn MaxWrappedBuilder<Self>) -> Self {
            //can call closure
            builder.add_inlet(MaxInlet::Float(Box::new(|_s, v| {
                post!("got float {}", v);
            })));
            //also can call method
            builder.add_inlet(MaxInlet::Int(Box::new(Self::int)));
            let _ = builder.add_inlet(MaxInlet::Proxy);
            Self {
                value: Default::default(),
                fvalue: Default::default(),
                _v: String::from("blah"),
                clock: builder.with_clockfn(Self::clocked),
                list_out: builder.add_list_outlet(),
            }
        }

        /// Register any methods you need for your class
        fn class_setup(c: &mut Class<MaxObjWrapper<Self>>) {
            c.add_method(median::method::Method::Int(Self::int_tramp)).expect("failed to add int method");
            c.add_method(median::method::Method::Bang(Self::bang_tramp)).expect("failed to add bang method");

            c.add_attribute(
                AttrBuilder::new_accessors(
                    "blah",
                    AttrType::Int64,
                    Self::blah_tramp,
                    Self::set_blah_tramp,
                )
                .build()
                .unwrap(),
            )
                .expect("failed to add attribute");

            c.add_attribute(
                AttrBuilder::new_accessors(
                    "foo",
                    AttrType::Float64,
                    Self::foo_tramp,
                    Self::set_foo_tramp,
                )
                .build()
                .unwrap(),
            )
                .expect("failed to add attribute");
        }
    }

    impl Simp {
        #[tramp]
        pub fn bang(&self) {
            let i = median::inlet::Proxy::get_inlet(self.max_obj());
            median::object_post!(self.max_obj(), "from rust {} inlet {}", self.value, i);
            self.clock.delay(10);
        }

        #[tramp]
        pub fn int(&self, v: max_sys::t_atom_long) {
            let i = median::inlet::Proxy::get_inlet(self.max_obj());
            self.value.set(v);
            median::attr::touch_with_name(self.max_obj(), SymbolRef::try_from("blah").unwrap())
                .unwrap();
            //XXX won't compile, needs mutex
            //self._v = format!("from rust {}", self.value);
            post!("from rust {} inlet {}", self.value, i);
        }

        #[attr_get_tramp]
        pub fn foo(&self) -> f64 {
            self.fvalue.get()
        }

        #[attr_set_tramp]
        pub fn set_foo(&self, v: f64) {
            self.fvalue.set(v);
        }

        #[attr_get_tramp]
        pub fn blah(&self) -> max_sys::t_atom_long {
            self.value.get()
        }

        #[attr_set_tramp]
        pub fn set_blah(&self, v: max_sys::t_atom_long) {
            self.value.set(v);
        }

        pub fn clocked(&self) {
            post("clocked".to_string());
            let _ = self.list_out.send(&[
                1i64.into(),
                12f64.into(),
                SymbolRef::try_from("foo").unwrap().into(),
            ]);
        }
    }
}
