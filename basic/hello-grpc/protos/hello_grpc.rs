// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_HELLO_SERVICE_SAY: ::grpcio::Method<super::hello::SayRequest, super::hello::SayResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/hello.HelloService/say",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct HelloServiceClient {
    client: ::grpcio::Client,
}

impl HelloServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        HelloServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn say_opt(&self, req: &super::hello::SayRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::hello::SayResponse> {
        self.client.unary_call(&METHOD_HELLO_SERVICE_SAY, req, opt)
    }

    pub fn say(&self, req: &super::hello::SayRequest) -> ::grpcio::Result<super::hello::SayResponse> {
        self.say_opt(req, ::grpcio::CallOption::default())
    }

    pub fn say_async_opt(&self, req: &super::hello::SayRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::hello::SayResponse>> {
        self.client.unary_call_async(&METHOD_HELLO_SERVICE_SAY, req, opt)
    }

    pub fn say_async(&self, req: &super::hello::SayRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::hello::SayResponse>> {
        self.say_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait HelloService {
    fn say(&mut self, ctx: ::grpcio::RpcContext, _req: super::hello::SayRequest, sink: ::grpcio::UnarySink<super::hello::SayResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_hello_service<S: HelloService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_HELLO_SERVICE_SAY, move |ctx, req, resp| {
        instance.say(ctx, req, resp)
    });
    builder.build()
}
