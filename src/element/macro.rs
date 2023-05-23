//! This module contains the macro for creating elements.

macro_rules! element {
    () => {
        // TODO(appcypher): Implement this.
    };
}

/// This macro implements the `Element` trait.
#[macro_export]
macro_rules! impl_element_trait {
    ($name:ident) => {
        impl Element for $name {
            fn get_base(&self) -> &std::sync::Mutex<BaseElement> {
                &self.base
            }
        }
    };
}

/// This macro creates builder pattern.
#[macro_export]
macro_rules! create_element_builder_struct {
    ($type:ident, $builder:ident) => {
        #[doc = concat!("The builder pattern for `", stringify!($type), "`.")]
        #[derive(Debug, Clone)]
        pub struct $builder(std::sync::Arc<$type>);
    };
}

/// This macro creates builder pattern.
#[macro_export]
macro_rules! impl_element_builder_methods {
    ($type:ident, $builder:ident) => {
        /// The builder pattern for the element.
        impl $builder {
            #[doc = concat!("Creates a new `",  stringify!($type),  "`.")]
            pub fn new() -> Self {
                Self(Arc::new($type {
                    base: Default::default(),
                }))
            }

            /// Adds an attribute to the element.
            pub fn attr(self, attr: Attribute) -> Self {
                self.0.add_attr(attr);
                self
            }

            /// Adds a child to the element.
            pub fn child(self, child: std::sync::Arc<dyn Element>) -> Self {
                self.0.append_child(child);
                self
            }

            /// Sets the content of the element.
            pub fn content(self, content: impl Into<String>) -> Self {
                self.0.set_content(content);
                self
            }

            /// Sets the content of the element.
            pub fn content_rx(
                self,
                content_observable: &impl crate::reactive::Observable<String>,
            ) -> Self {
                self.0.set_content_rx(content_observable);
                self
            }

            /// Builds the element.
            pub fn build(self) -> std::sync::Arc<$type> {
                self.0
            }
        }
    };
}

/// This macro implements element helper methods.
#[macro_export]
macro_rules! impl_element_methods {
    ($name:ident) => {
        impl $name {
            /// Adds an attribute to the element.
            fn add_attr(&self, attr: Attribute) {
                self.base.lock().unwrap().attributes.push(attr);
            }

            /// Adds a child to the element.
            fn append_child(self: &Arc<Self>, child: std::sync::Arc<dyn Element>) {
                {
                    let mut child_base = child.get_base().lock().unwrap();
                    child_base.parent = Some({
                        let tmp = Arc::downgrade(&self);
                        tmp
                    });
                    child_base.index = self.base.lock().unwrap().children.len() as u16;
                }

                self.base.lock().unwrap().children.push(child);
            }

            /// Sets the content of the element.
            fn set_content(&self, content: impl Into<String>) {
                self.base.lock().unwrap().content = content.into();
            }

            /// Sets the content of the element.
            fn set_content_rx(
                self: &Arc<Self>,
                content_observable: &impl crate::reactive::Observable<String>,
            ) {
                let clone = Arc::clone(self);
                content_observable.add_subscriber(std::sync::Arc::new(
                    move |content: &String| -> anyhow::Result<()> {
                        clone.base.lock().unwrap().content = content.clone();
                        Ok(())
                    },
                ));
            }
        }
    };
}
