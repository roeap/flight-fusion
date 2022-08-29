// @generated
/// Generated server implementations.
pub mod mlflow_artifacts_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with MlflowArtifactsServiceServer.
    #[async_trait]
    pub trait MlflowArtifactsService: Send + Sync + 'static {
        ///Server streaming response type for the downloadArtifact method.
        type downloadArtifactStream: futures_core::Stream<
                Item = Result<super::download_artifact::Response, tonic::Status>,
            >
            + Send
            + 'static;
        async fn download_artifact(
            &self,
            request: tonic::Request<super::DownloadArtifact>,
        ) -> Result<tonic::Response<Self::downloadArtifactStream>, tonic::Status>;
        async fn upload_artifact(
            &self,
            request: tonic::Request<tonic::Streaming<super::UploadArtifact>>,
        ) -> Result<tonic::Response<super::upload_artifact::Response>, tonic::Status>;
        async fn list_artifacts(
            &self,
            request: tonic::Request<super::ListArtifacts>,
        ) -> Result<tonic::Response<super::list_artifacts::Response>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MlflowArtifactsServiceServer<T: MlflowArtifactsService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MlflowArtifactsService> MlflowArtifactsServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for MlflowArtifactsServiceServer<T>
    where
        T: MlflowArtifactsService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/mlflow.artifacts.MlflowArtifactsService/downloadArtifact" => {
                    #[allow(non_camel_case_types)]
                    struct downloadArtifactSvc<T: MlflowArtifactsService>(pub Arc<T>);
                    impl<
                        T: MlflowArtifactsService,
                    > tonic::server::ServerStreamingService<super::DownloadArtifact>
                    for downloadArtifactSvc<T> {
                        type Response = super::download_artifact::Response;
                        type ResponseStream = T::downloadArtifactStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DownloadArtifact>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).download_artifact(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = downloadArtifactSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.artifacts.MlflowArtifactsService/uploadArtifact" => {
                    #[allow(non_camel_case_types)]
                    struct uploadArtifactSvc<T: MlflowArtifactsService>(pub Arc<T>);
                    impl<
                        T: MlflowArtifactsService,
                    > tonic::server::ClientStreamingService<super::UploadArtifact>
                    for uploadArtifactSvc<T> {
                        type Response = super::upload_artifact::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::UploadArtifact>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).upload_artifact(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = uploadArtifactSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/mlflow.artifacts.MlflowArtifactsService/listArtifacts" => {
                    #[allow(non_camel_case_types)]
                    struct listArtifactsSvc<T: MlflowArtifactsService>(pub Arc<T>);
                    impl<
                        T: MlflowArtifactsService,
                    > tonic::server::UnaryService<super::ListArtifacts>
                    for listArtifactsSvc<T> {
                        type Response = super::list_artifacts::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListArtifacts>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_artifacts(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = listArtifactsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: MlflowArtifactsService> Clone for MlflowArtifactsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: MlflowArtifactsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MlflowArtifactsService> tonic::server::NamedService
    for MlflowArtifactsServiceServer<T> {
        const NAME: &'static str = "mlflow.artifacts.MlflowArtifactsService";
    }
}
