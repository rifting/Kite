use std::path::PathBuf;

struct ExternalCert {
    cert: PathBuf,
    private_key: PathBuf,
}

pub mod proxy {
    use http_body_util::BodyExt;
    use http_mitm_proxy::{moka::sync::Cache, DefaultClient, MitmProxy};
    use hyper::{body::Incoming, service::service_fn, Request};
    use kite::Data;
    use rcgen::CertifiedKey;

    use crate::proxy::ExternalCert;

    pub async fn start_proxy(config: Data) {
        let port = config.proxy.port;
        let ip = config.proxy.ip;

        let kite_port = config.server.port;
        let kite_ip = config.server.ip;

        let root_cert: CertifiedKey = if config.proxy.cert != "" && config.proxy.private_key != "" {
            let certy = ExternalCert {
                cert: config.proxy.cert.clone().into(),
                private_key: config.proxy.private_key.clone().into()
            };


            // Use existing key
            let param = rcgen::CertificateParams::from_ca_cert_pem(
                &std::fs::read_to_string(&certy.cert).unwrap(),
            )
            .unwrap();
            let key_pair =
                rcgen::KeyPair::from_pem(&std::fs::read_to_string(&certy.private_key).unwrap())
                    .unwrap();

            let cert = param.self_signed(&key_pair).unwrap();

            rcgen::CertifiedKey { cert, key_pair }
        } else {
            make_root_cert()
        };

        let root_cert_pem = root_cert.cert.pem();
        let root_cert_key = root_cert.key_pair.serialize_pem();

        let proxy = MitmProxy::new(
            // This is the root cert that will be used to sign the fake certificates
            Some(root_cert),
            Some(Cache::new(128)),
        );

        let client = DefaultClient::new();
        let proxy = proxy
            .bind(
                (ip.clone(), port),
                service_fn(move |mut req| {
                    let client = client.clone();
                    let kite_ip = kite_ip.clone();
                    async move {
                        // Redirect to the Kite server

                        // Transparency mode
                        // If enabled, only redirect for the classifyUrl path.
                        if !config.proxy.transparency {
                            if req.uri().host() == Some("kidsmanagement-pa.googleapis.com") {
                                rewrite_request(&mut req, &kite_ip, kite_port);
                            }
                        } else {
                            if req.uri().host() == Some("kidsmanagement-pa.googleapis.com") &&
                               req.uri().path().starts_with("/kidsmanagement/v1/people/me:classifyUrl") {
                                rewrite_request(&mut req, &kite_ip, kite_port);
                            }
                        }
                        
                        let (res, _upgrade) = client.send_request(req).await?;

                        Ok::<_, http_mitm_proxy::default_client::Error>(res.map(|b| b.boxed()))
                    }
                }),
            )
            .await
            .unwrap();

        println!("");
        println!("Kite's HTTP Proxy is listening on http://{}:{}", ip, config.proxy.port);

        if config.proxy.cert == "" && config.proxy.private_key == "" {
            println!("");
            println!("You didn't provide a private key and cert for Kite to use.");
            println!("As such, Kite has automatically generated them for you.");
            println!();
            println!("Trust this cert if you want to use HTTPS");
            println!();
            println!("{}", root_cert_pem);
            println!();
            println!("Private key");
            println!("{}", root_cert_key);
        }

        proxy.await;
    }

    fn make_root_cert() -> rcgen::CertifiedKey {
        let mut param = rcgen::CertificateParams::default();
    
        param.distinguished_name = rcgen::DistinguishedName::new();
        param.distinguished_name.push(
            rcgen::DnType::CommonName,
            rcgen::DnValue::Utf8String("Kite Server".to_string()),
        );
        param.key_usages = vec![
            rcgen::KeyUsagePurpose::KeyCertSign,
            rcgen::KeyUsagePurpose::CrlSign,
        ];
        param.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    
        let key_pair = rcgen::KeyPair::generate().unwrap();
        let cert = param.self_signed(&key_pair).unwrap();
    
        rcgen::CertifiedKey { cert, key_pair }
    } 

    fn rewrite_request(req: &mut Request<Incoming>, kite_ip: &str, kite_port: u16) {
        req.headers_mut().insert(
            hyper::header::HOST,
            hyper::header::HeaderValue::from_maybe_shared(format!("{}:{}", kite_ip, kite_port))
                .unwrap(),
        );

        let mut parts = req.uri().clone().into_parts();
        parts.scheme = Some(hyper::http::uri::Scheme::HTTP);
        parts.authority = Some(
            hyper::http::uri::Authority::from_maybe_shared(format!("{}:{}", kite_ip, kite_port))
                .unwrap(),
        );
        parts.path_and_query = Some(
            parts
                .path_and_query
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
        );
        *req.uri_mut() = hyper::Uri::from_parts(parts).unwrap();
    }
    
}   