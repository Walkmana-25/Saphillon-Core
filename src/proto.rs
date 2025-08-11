// Re-export the generated protobuf code

pub mod sapphillon {
    #![allow(clippy::all)]
    
    pub mod v1 {
        include!("proto/sapphillon.v1.rs");
    }
}

pub mod google {
    #![allow(clippy::all)]
    #![cfg(not(doctest))]
    
    pub mod api {
        include!("proto/google.api.rs");
        pub mod expr {
            pub mod v1alpha1 {
                include!("proto/google.api.expr.v1alpha1.rs");
            }
            pub mod v1beta1 {
                include!("proto/google.api.expr.v1beta1.rs");
            }
        }
    }
    pub mod bytestream {
        include!("proto/google.bytestream.rs");
    }
    pub mod longrunning {
        include!("proto/google.longrunning.rs");
    }
    pub mod geo {
        pub mod r#type {
            include!("proto/google.geo.type.rs");
        }
    }
    pub mod rpc {
        include!("proto/google.rpc.rs");
        pub mod context {
            include!("proto/google.rpc.context.rs");
        }
    }
    pub mod r#type {
        include!("proto/google.type.rs");
    }
}
